use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::json;
use webkit::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn check(condition: bool, what: &str) -> TestResult {
    if condition {
        Ok(())
    } else {
        Err(format!("check failed: {what}").into())
    }
}

fn check_eq(actual: &str, expected: &str, what: &str) -> TestResult {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{what}: expected {expected:?}, got {actual:?}").into())
    }
}

const fn is_invalid_state(result: &Result<(), WebKitError>) -> bool {
    matches!(result, Err(WebKitError::InvalidState(_)))
}

fn scratch_dir(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("main-thread-tests")
        .join(name);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn configuration() -> WebViewConfiguration {
    let config = WebViewConfiguration::new();
    config.use_nonpersistent_data_store();
    config.set_preferences(&Preferences {
        fraudulent_website_warning_enabled: false,
        ..Preferences::default()
    });
    config
}

fn pump_until(timeout: Duration, mut done: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if done() {
            return true;
        }
        webkit::pump_run_loop(0.02);
    }
    done()
}

fn html_response(task: &UrlSchemeTask) -> UrlSchemeResponse {
    UrlSchemeResponse::new(task.request().url.clone(), "text/html")
        .with_text_encoding_name("utf-8")
        .with_header("Content-Type", "text/html; charset=utf-8")
}

fn respond_html(task: &UrlSchemeTask, html: &str) {
    let _ = task.respond(&html_response(task), html.as_bytes());
}

struct Scheme<F> {
    start: F,
    stops: Arc<AtomicUsize>,
}

impl<F> Scheme<F>
where
    F: Fn(UrlSchemeTask) + Send + Sync + 'static,
{
    fn new(start: F) -> Self {
        Self {
            start,
            stops: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<F> UrlSchemeHandler for Scheme<F>
where
    F: Fn(UrlSchemeTask) + Send + Sync + 'static,
{
    fn start(&self, task: UrlSchemeTask) {
        (self.start)(task);
    }

    fn stop(&self, _task: UrlSchemeTask) {
        self.stops.fetch_add(1, Ordering::SeqCst);
    }
}

fn collect_messages(view: &mut WebView) -> Arc<Mutex<Vec<ScriptMessage>>> {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&messages);
    view.set_message_handler(move |message| lock(&sink).push(message.clone()));
    messages
}

fn url_scheme_task_rejects_out_of_order_calls() -> TestResult {
    let outcomes = Arc::new(Mutex::new(Vec::<(&'static str, bool)>::new()));
    let log = Arc::clone(&outcomes);
    let config = configuration();
    config.set_url_scheme_handler(
        "b28order",
        Scheme::new(move |task: UrlSchemeTask| {
            let response = html_response(&task);
            let mut log = lock(&log);
            log.push((
                "data before a response",
                is_invalid_state(&task.did_receive_data(b"early")),
            ));
            log.push((
                "finish before a response",
                is_invalid_state(&task.did_finish()),
            ));
            log.push((
                "first response",
                task.did_receive_response(&response).is_ok(),
            ));
            log.push((
                "second response before data",
                task.did_receive_response(&response).is_ok(),
            ));
            log.push((
                "data",
                task.did_receive_data(b"<p id='ok'>ordered</p>").is_ok(),
            ));
            log.push((
                "response after data",
                is_invalid_state(&task.did_receive_response(&response)),
            ));
            log.push(("finish", task.did_finish().is_ok()));
            log.push(("second finish", is_invalid_state(&task.did_finish())));
            log.push((
                "fail after finish",
                is_invalid_state(&task.did_fail("late")),
            ));
            log.push((
                "data after finish",
                is_invalid_state(&task.did_receive_data(b"late")),
            ));
        }),
    )?;
    let view = WebView::with_config(&config)?;
    view.load_url("b28order://host/index")?;
    check_eq(
        &view.evaluate_javascript("document.getElementById('ok').textContent")?,
        "ordered",
        "page served after the rejected calls",
    )?;
    let outcomes = lock(&outcomes).clone();
    check(outcomes.len() == 10, "every step ran")?;
    for (step, passed) in &outcomes {
        check(*passed, step)?;
    }
    Ok(())
}

fn url_scheme_task_fail_without_response_is_allowed_once() -> TestResult {
    let outcomes = Arc::new(Mutex::new(Vec::<(&'static str, bool)>::new()));
    let log = Arc::clone(&outcomes);
    let config = configuration();
    config.set_url_scheme_handler(
        "b28fail",
        Scheme::new(move |task: UrlSchemeTask| {
            if task.request().url.ends_with("/index") {
                respond_html(&task, "<p>index</p>");
                return;
            }
            let mut log = lock(&log);
            log.push(("fail without a response", task.did_fail("refused").is_ok()));
            log.push(("second fail", is_invalid_state(&task.did_fail("again"))));
            log.push((
                "response after fail",
                is_invalid_state(&task.did_receive_response(&html_response(&task))),
            ));
        }),
    )?;
    let view = WebView::with_config(&config)?;
    view.load_url("b28fail://host/index")?;
    let outcome = view.call_async_javascript(
        "try { await fetch('b28fail://host/refused'); return 'resolved'; } catch (error) { return 'rejected'; }",
        &json!({}),
        None,
        &ContentWorld::Page,
    )?;
    check_eq(&outcome, "rejected", "failed task rejects the fetch")?;
    let outcomes = lock(&outcomes).clone();
    check(outcomes.len() == 3, "every step ran")?;
    for (step, passed) in &outcomes {
        check(*passed, step)?;
    }
    Ok(())
}

fn url_scheme_task_calls_after_stop_return_errors() -> TestResult {
    let pending = Arc::new(Mutex::new(None::<UrlSchemeTask>));
    let slot = Arc::clone(&pending);
    let handler = Scheme::new(move |task: UrlSchemeTask| {
        if task.request().url.ends_with("/pending") {
            *lock(&slot) = Some(task);
        } else {
            respond_html(&task, "<p>index</p>");
        }
    });
    let stops = Arc::clone(&handler.stops);
    let config = configuration();
    config.set_url_scheme_handler("b28stop", handler)?;
    let view = WebView::with_config(&config)?;
    view.load_url("b28stop://host/index")?;
    view.evaluate_javascript("fetch('b28stop://host/pending').catch(() => {}); 1")?;
    check(
        pump_until(Duration::from_secs(5), || lock(&pending).is_some()),
        "the pending request reached the handler",
    )?;
    view.load_html("<p>away</p>", None)?;
    check(
        pump_until(Duration::from_secs(5), || stops.load(Ordering::SeqCst) > 0),
        "WebKit stopped the pending task",
    )?;
    let task = lock(&pending).take().ok_or("pending task")?;
    check(
        is_invalid_state(&task.did_receive_response(&html_response(&task))),
        "response after stop",
    )?;
    check(
        is_invalid_state(&task.did_receive_data(b"x")),
        "data after stop",
    )?;
    check(is_invalid_state(&task.did_finish()), "finish after stop")?;
    check(is_invalid_state(&task.did_fail("gone")), "fail after stop")?;
    drop(task);
    check_eq(
        &view.evaluate_javascript("document.body.textContent")?,
        "away",
        "the web view keeps working",
    )
}

fn url_scheme_handler_receives_request_bodies() -> TestResult {
    let requests = Arc::new(Mutex::new(Vec::<UrlSchemeRequest>::new()));
    let log = Arc::clone(&requests);
    let config = configuration();
    config.set_url_scheme_handler(
        "b28body",
        Scheme::new(move |task: UrlSchemeTask| {
            lock(&log).push(task.request().clone());
            respond_html(&task, "<p>ok</p>");
        }),
    )?;
    let view = WebView::with_config(&config)?;
    view.load_url("b28body://host/index")?;
    view.call_async_javascript(
        "const response = await fetch('b28body://host/submit', { method: 'POST', body: payload }); return await response.text();",
        &json!({ "payload": "b28-payload" }),
        None,
        &ContentWorld::Page,
    )?;
    let requests = lock(&requests).clone();
    let post = requests
        .iter()
        .find(|request| request.url.ends_with("/submit"))
        .ok_or("POST request reached the handler")?;
    check_eq(&post.method, "POST", "request method")?;
    check(
        post.body.as_deref() == Some(b"b28-payload".as_slice()),
        "request body is delivered",
    )?;
    let index = requests
        .iter()
        .find(|request| request.url.ends_with("/index"))
        .ok_or("GET request reached the handler")?;
    check(index.body.is_none(), "GET requests carry no body")
}

fn script_messages_carry_frame_origin_and_world() -> TestResult {
    let config = configuration();
    config.set_url_scheme_handler(
        "b28msg",
        Scheme::new(|task: UrlSchemeTask| {
            let html = if task.request().url.starts_with("b28msg://main/") {
                "<script>window.webkit.messageHandlers.bridge.postMessage('from-main')</script><iframe src='b28msg://other/frame'></iframe>"
            } else {
                "<script>window.webkit.messageHandlers.bridge.postMessage('from-frame')</script>"
            };
            respond_html(&task, html);
        }),
    )?;
    config.add_message_handler("bridge", &ContentWorld::Page)?;
    let mut view = WebView::with_config(&config)?;
    let live = collect_messages(&mut view);
    view.load_url("b28msg://main/index")?;
    check(
        pump_until(Duration::from_secs(5), || lock(&live).len() >= 2),
        "both frames posted a message",
    )?;
    let messages = lock(&live).clone();
    let main = messages
        .iter()
        .find(|message| message.body == "from-main")
        .ok_or("main-frame message")?;
    let frame = messages
        .iter()
        .find(|message| message.body == "from-frame")
        .ok_or("iframe message")?;
    check(main.frame.main_frame, "main frame flag")?;
    check(!frame.frame.main_frame, "iframe flag")?;
    check_eq(
        &main.frame.security_origin.protocol,
        "b28msg",
        "main origin protocol",
    )?;
    check_eq(&main.frame.security_origin.host, "main", "main origin host")?;
    check_eq(
        &frame.frame.security_origin.host,
        "other",
        "iframe origin host",
    )?;
    check(
        frame.frame.request_url.starts_with("b28msg://other/"),
        "iframe request URL",
    )?;
    check(main.world == ContentWorld::Page, "page content world")?;
    check(
        main.frame_handle.is_some() && frame.frame_handle.is_some(),
        "live messages carry frame handles",
    )?;

    let drained = view.drain_script_messages();
    check(drained.events.len() == 2, "queued copies of both messages")?;
    check(drained.dropped == 0, "nothing dropped")?;
    check(
        drained.events.iter().any(|message| {
            !message.frame.main_frame && message.frame.security_origin.host == "other"
        }),
        "queued messages keep the origin",
    )?;

    let handle = frame.frame_handle.as_ref().ok_or("iframe handle")?;
    let host = view.call_async_javascript(
        "return location.host",
        &json!({}),
        Some(handle),
        &ContentWorld::Page,
    )?;
    check_eq(&host, "other", "callAsyncJavaScript targets the iframe")
}

fn isolated_world_handler_is_hidden_from_page_scripts() -> TestResult {
    let world = ContentWorld::Named("b28-host".to_owned());
    let config = configuration();
    config.add_message_handler("secure", &world)?;
    config.add_user_script(
        &UserScript::new("window.webkit.messageHandlers.secure.postMessage('from-isolated-world')")
            .with_content_world("b28-host"),
    );
    let mut view = WebView::with_config(&config)?;
    let live = collect_messages(&mut view);
    view.load_html("<p>page</p>", Some("https://b28-world.test/"))?;
    check(
        pump_until(Duration::from_secs(5), || !lock(&live).is_empty()),
        "the isolated-world script posted",
    )?;
    check_eq(
        &view.evaluate_javascript("typeof window.webkit?.messageHandlers?.secure")?,
        "undefined",
        "page scripts cannot see the handler",
    )?;
    check_eq(
        &view.call_async_javascript(
            "return typeof window.webkit.messageHandlers.secure",
            &json!({}),
            None,
            &ContentWorld::Named("b28-host".to_owned()),
        )?,
        "object",
        "the isolated world can see the handler",
    )?;
    let messages = lock(&live);
    check(
        messages.len() == 1,
        "only the isolated-world message arrived",
    )?;
    check(
        messages[0].world == world,
        "message world is the isolated world",
    )
}

fn duplicate_handler_names_are_rejected() -> TestResult {
    let other = ContentWorld::Named("b28-other".to_owned());
    let config = configuration();
    config.add_message_handler("dup", &ContentWorld::Page)?;
    check(
        matches!(
            config.add_message_handler("dup", &ContentWorld::Page),
            Err(WebKitError::InvalidArgument(_))
        ),
        "the same plain handler twice",
    )?;
    check(
        matches!(
            config.add_message_handler_with_reply("dup", &ContentWorld::Page),
            Err(WebKitError::InvalidArgument(_))
        ),
        "a reply handler reusing a plain handler name",
    )?;
    config.add_message_handler_with_reply("dup", &other)?;
    check(
        config.add_message_handler("dup", &other).is_err(),
        "a plain handler reusing a reply handler name",
    )?;
    check(
        config.add_message_handler("", &ContentWorld::Page).is_err(),
        "empty handler name",
    )?;
    check(
        config
            .add_message_handler("named", &ContentWorld::Named(String::new()))
            .is_err(),
        "empty world name",
    )?;
    config.remove_message_handler("dup", &ContentWorld::Page)?;
    check(
        config
            .remove_message_handler("dup", &ContentWorld::Page)
            .is_err(),
        "removing a handler twice",
    )?;
    config.add_message_handler("dup", &ContentWorld::Page)?;

    let first = WebView::with_config(&config)?;
    let second = WebView::with_config(&config)?;
    drop((first, second));
    webkit::pump_run_loop(0.05);
    Ok(())
}

fn web_views_sharing_a_configuration_get_their_own_messages() -> TestResult {
    let config = configuration();
    config.add_message_handler("shared", &ContentWorld::Page)?;
    let mut first = WebView::with_config(&config)?;
    let mut second = WebView::with_config(&config)?;
    let first_messages = collect_messages(&mut first);
    let second_messages = collect_messages(&mut second);
    first.load_html(
        "<script>window.webkit.messageHandlers.shared.postMessage('first')</script>",
        None,
    )?;
    second.load_html(
        "<script>window.webkit.messageHandlers.shared.postMessage('second')</script>",
        None,
    )?;
    check(
        pump_until(Duration::from_secs(5), || {
            !lock(&first_messages).is_empty() && !lock(&second_messages).is_empty()
        }),
        "both web views received a message",
    )?;
    webkit::pump_run_loop(0.1);
    let first_bodies: Vec<String> = lock(&first_messages)
        .iter()
        .map(|m| m.body.clone())
        .collect();
    let second_bodies: Vec<String> = lock(&second_messages)
        .iter()
        .map(|m| m.body.clone())
        .collect();
    check(
        first_bodies == ["first"],
        "first web view got only its message",
    )?;
    check(
        second_bodies == ["second"],
        "second web view got only its message",
    )
}

fn reply_handler_can_reject_cross_origin_frames() -> TestResult {
    let config = configuration();
    config.set_url_scheme_handler(
        "b28reply",
        Scheme::new(|task: UrlSchemeTask| {
            let html = if task.request().url.starts_with("b28reply://main/") {
                "<iframe src='b28reply://other/frame'></iframe>"
            } else {
                "<script>window.webkit.messageHandlers.hello.postMessage('ready')</script>"
            };
            respond_html(&task, html);
        }),
    )?;
    config.add_message_handler("hello", &ContentWorld::Page)?;
    config.add_message_handler_with_reply("reply", &ContentWorld::Page)?;
    let mut view = WebView::with_config(&config)?;
    let hello = collect_messages(&mut view);
    view.set_message_handler_with_reply(|message| {
        if message.frame.main_frame && message.frame.security_origin.host == "main" {
            Ok(Some(json!("pong")))
        } else {
            Err(WebKitError::InvalidArgument(format!(
                "rejected {}",
                message.frame.security_origin.host
            )))
        }
    });
    view.load_url("b28reply://main/index")?;
    check(
        pump_until(Duration::from_secs(5), || !lock(&hello).is_empty()),
        "the iframe announced itself",
    )?;
    let reply = view.call_async_javascript(
        "return await window.webkit.messageHandlers.reply.postMessage('ping')",
        &json!({}),
        None,
        &ContentWorld::Page,
    )?;
    check_eq(&reply, "pong", "main frame reply")?;
    let frame = lock(&hello)[0]
        .frame_handle
        .clone()
        .ok_or("iframe handle")?;
    let rejected = view.call_async_javascript(
        "try { await window.webkit.messageHandlers.reply.postMessage('ping'); return 'accepted'; } catch (error) { return 'rejected: ' + error.message; }",
        &json!({}),
        Some(&frame),
        &ContentWorld::Page,
    )?;
    check(
        rejected.starts_with("rejected") && rejected.contains("other"),
        "cross-origin iframe was rejected",
    )
}

fn navigation_action_handler_decides_each_request() -> TestResult {
    let config = configuration();
    config.set_url_scheme_handler(
        "b28nav",
        Scheme::new(|task: UrlSchemeTask| respond_html(&task, "<p>scheme page</p>")),
    )?;
    let mut view = WebView::with_config(&config)?;
    let seen = Arc::new(Mutex::new(Vec::<NavigationAction>::new()));
    let sink = Arc::clone(&seen);
    view.set_navigation_action_handler(move |action| {
        lock(&sink).push(action.clone());
        if action.request_url.starts_with("b28nav://allowed/") {
            NavigationActionPolicy::Allow
        } else {
            NavigationActionPolicy::Cancel
        }
    });
    view.load_url("b28nav://allowed/start")?;
    check_eq(&view.url(), "b28nav://allowed/start", "allowed load")?;

    view.evaluate_javascript("location.href = 'b28nav://blocked/page'; 1")?;
    check(
        pump_until(Duration::from_secs(5), || {
            lock(&seen)
                .iter()
                .any(|action| action.request_url.starts_with("b28nav://blocked/"))
        }),
        "the handler saw the page-initiated navigation",
    )?;
    webkit::pump_run_loop(0.3);
    check_eq(&view.url(), "b28nav://allowed/start", "blocked navigation")?;
    let blocked = lock(&seen)
        .iter()
        .find(|action| action.request_url.starts_with("b28nav://blocked/"))
        .cloned()
        .ok_or("blocked action")?;
    check(
        blocked
            .target_frame
            .as_ref()
            .is_some_and(|frame| frame.main_frame),
        "target frame is the main frame",
    )?;
    check_eq(
        &blocked.source_frame.security_origin.host,
        "allowed",
        "source origin",
    )?;
    check(
        blocked.navigation_type == NavigationType::Other,
        "script navigation type",
    )?;

    check(
        view.load_url("b28nav://blocked/direct").is_err(),
        "a cancelled load reports an error",
    )?;
    view.set_navigation_action_handler(|_| panic!("navigation policy handler panicked"));
    check(
        view.load_url("b28nav://allowed/after-panic").is_err(),
        "a panicking handler cancels the navigation",
    )?;
    check_eq(
        &view.url(),
        "b28nav://allowed/start",
        "still on the start page",
    )
}

fn navigation_response_handler_decides_each_response() -> TestResult {
    let config = configuration();
    config.set_url_scheme_handler(
        "b28resp",
        Scheme::new(|task: UrlSchemeTask| respond_html(&task, "<p>response page</p>")),
    )?;
    let mut view = WebView::with_config(&config)?;
    view.set_navigation_response_handler(|response| {
        if response.url.contains("/refused") {
            NavigationResponsePolicy::Cancel
        } else {
            NavigationResponsePolicy::Allow
        }
    });
    view.load_url("b28resp://host/accepted")?;
    check(
        view.load_url("b28resp://host/refused").is_err(),
        "a refused response reports an error",
    )?;
    check_eq(
        &view.url(),
        "b28resp://host/accepted",
        "refused response did not commit",
    )
}

fn call_async_javascript_uses_arguments_and_worlds() -> TestResult {
    let view = WebView::with_config(&configuration())?;
    view.load_html("<p>js</p>", Some("https://b28-js.test/"))?;
    let hostile = "\"; throw new Error('injected'); //</script>";
    let echoed = view.call_async_javascript(
        "return value",
        &json!({ "value": hostile }),
        None,
        &ContentWorld::Page,
    )?;
    check_eq(&echoed, hostile, "untrusted text arrives as data")?;
    let sum = view.call_async_javascript(
        "return a + b",
        &json!({ "a": 2, "b": 3 }),
        None,
        &ContentWorld::Page,
    )?;
    check_eq(&sum, "5", "numeric arguments")?;
    view.evaluate_javascript("window.pageSecret = 42; 1")?;
    let page = view.call_async_javascript(
        "return typeof window.pageSecret",
        &json!({}),
        None,
        &ContentWorld::Page,
    )?;
    check_eq(&page, "number", "page world sees page globals")?;
    let named = view.call_async_javascript(
        "return typeof window.pageSecret",
        &json!({}),
        None,
        &ContentWorld::Named("b28-js".to_owned()),
    )?;
    check_eq(&named, "undefined", "named world is isolated")?;
    let client = view.call_async_javascript(
        "return typeof window.pageSecret",
        &json!(null),
        None,
        &ContentWorld::DefaultClient,
    )?;
    check_eq(&client, "undefined", "default client world is isolated")?;
    check(
        matches!(
            view.call_async_javascript("return 1", &json!([1]), None, &ContentWorld::Page),
            Err(WebKitError::InvalidArgument(_))
        ),
        "non-object arguments are rejected",
    )
}

fn event_queues_drop_the_oldest_entries() -> TestResult {
    let config = configuration();
    config.add_message_handler("flood", &ContentWorld::Page)?;
    let mut view = WebView::with_config(&config)?;
    let delivered = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&delivered);
    view.set_message_handler(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    });
    view.load_html("<p>flood</p>", Some("https://b28-flood.test/"))?;
    view.evaluate_javascript(
        "for (let i = 0; i < 3000; i++) { window.webkit.messageHandlers.flood.postMessage(String(i)); } 1",
    )?;
    check(
        pump_until(Duration::from_secs(20), || {
            delivered.load(Ordering::SeqCst) >= 3000
        }),
        "every message reached the live handler",
    )?;
    let drained = view.drain_script_messages();
    check(
        drained.events.len() == 1024,
        "the queue keeps at most 1024 messages",
    )?;
    check(
        drained.dropped == 3000 - 1024,
        "the queue reports what it dropped",
    )?;
    check_eq(
        &drained.events.last().ok_or("newest message")?.body,
        "2999",
        "the newest message is kept",
    )?;
    check_eq(
        &drained.events[0].body,
        "1976",
        "the oldest messages are dropped first",
    )?;

    view.evaluate_javascript(
        "window.webkit.messageHandlers.flood.postMessage('x'.repeat(5 * 1024 * 1024)); 1",
    )?;
    check(
        pump_until(Duration::from_secs(10), || {
            delivered.load(Ordering::SeqCst) >= 3001
        }),
        "the oversized message still reaches the live handler",
    )?;
    let oversized = view.drain_script_messages();
    check(
        oversized.events.is_empty() && oversized.dropped == 1,
        "an oversized message is not queued",
    )?;

    view.evaluate_javascript("for (let i = 0; i < 1500; i++) { alert(String(i)); } 1")?;
    let alerts = view.drain_ui_events();
    check(
        alerts.events.len() == 1024,
        "the UI queue keeps at most 1024 events",
    )?;
    check(
        alerts.dropped == 1500 - 1024,
        "the UI queue reports what it dropped",
    )?;
    check(
        alerts
            .events
            .last()
            .and_then(|event| event.message.as_deref())
            == Some("1499"),
        "the newest alert is kept",
    )
}

fn handlers_swap_atomically_and_are_released() -> TestResult {
    let config = configuration();
    config.add_message_handler("tick", &ContentWorld::Page)?;
    let mut view = WebView::with_config(&config)?;
    let first = Arc::new(AtomicUsize::new(0));
    let second = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&first);
    view.set_message_handler(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    });
    view.load_html(
        "<script>setInterval(() => window.webkit.messageHandlers.tick.postMessage('tick'), 5)</script>",
        Some("https://b28-swap.test/"),
    )?;
    check(
        pump_until(Duration::from_secs(5), || first.load(Ordering::SeqCst) >= 5),
        "the first handler receives messages",
    )?;
    let counter = Arc::clone(&second);
    view.set_message_handler(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    });
    let first_after_swap = first.load(Ordering::SeqCst);
    check(
        Arc::strong_count(&first) == 1,
        "the replaced handler was released",
    )?;
    check(
        pump_until(Duration::from_secs(5), || {
            second.load(Ordering::SeqCst) >= 5
        }),
        "the second handler receives messages",
    )?;
    check(
        first.load(Ordering::SeqCst) == first_after_swap,
        "the replaced handler no longer runs",
    )?;
    drop(view);
    let second_after_drop = second.load(Ordering::SeqCst);
    webkit::pump_run_loop(0.3);
    check(
        second.load(Ordering::SeqCst) == second_after_drop,
        "a dropped web view delivers nothing",
    )?;
    check(
        Arc::strong_count(&second) == 1,
        "a dropped web view released its handler",
    )
}

fn serve_attachment(
    body: &'static [u8],
    file_name: &'static str,
) -> Result<(String, JoinHandle<()>), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let url = format!("http://{}/download", listener.local_addr()?);
    let handle = thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut request = [0_u8; 2048];
            let _ = stream.read(&mut request);
            let headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nContent-Disposition: attachment; filename=\"{file_name}\"\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(headers.as_bytes());
            let _ = stream.write_all(body);
            let _ = stream.flush();
        }
    });
    Ok((url, handle))
}

fn download_file_names_cannot_escape_the_destination() -> TestResult {
    let root = scratch_dir("downloads")?.join(format!("run-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    let destination = root.join("inside");
    fs::create_dir_all(&destination)?;
    let view = WebView::with_config(&configuration())?;
    for suggested in ["../../b28-escape.txt", ".b28-hidden"] {
        let (url, server) = serve_attachment(b"b28 payload", suggested)?;
        let download = view.start_download_using_request(&url, &destination)?;
        let mut events = Vec::new();
        let completed = pump_until(Duration::from_secs(10), || {
            events.extend(download.drain_events().events);
            events
                .iter()
                .any(|event| event.kind == "finish" || event.kind == "fail")
        });
        let _ = server.join();
        check(completed, "the download completed")?;
        let finished = events
            .iter()
            .find(|event| event.kind == "finish")
            .ok_or("the download finished")?;
        let path = PathBuf::from(finished.destination.as_deref().ok_or("destination path")?);
        let parent = path.parent().ok_or("destination parent")?;
        check(
            fs::canonicalize(parent)? == fs::canonicalize(&destination)?,
            "the file stays inside the destination directory",
        )?;
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("file name")?;
        check(!name.starts_with('.'), "no hidden file names")?;
        check(
            fs::read(&path)? == b"b28 payload",
            "the payload was written",
        )?;
    }
    check(
        !root.join("b28-escape.txt").exists()
            && !scratch_dir("downloads")?.join("b28-escape.txt").exists(),
        "nothing was written outside the destination",
    )?;
    fs::remove_dir_all(&root)?;
    Ok(())
}

fn calls_from_other_threads_run_on_the_main_thread() -> TestResult {
    let worker = thread::spawn(|| {
        let config = WebViewConfiguration::new();
        config.set_application_name_for_user_agent("b28-worker");
        let name = config.application_name_for_user_agent();
        let store = WebsiteDataStore::non_persistent();
        (name, store.is_persistent())
    });
    check(
        pump_until(Duration::from_secs(10), || worker.is_finished()),
        "the worker finished while the main thread serviced its queue",
    )?;
    let (name, persistent) = worker.join().map_err(|_| "the worker panicked")?;
    check_eq(&name, "b28-worker", "configuration round trip")?;
    check(!persistent, "non-persistent store")?;

    let view = WebView::with_config(&configuration())?;
    let dropper = thread::spawn(move || drop(view));
    check(
        pump_until(Duration::from_secs(5), || dropper.is_finished()),
        "dropping a web view on another thread returns",
    )?;
    dropper.join().map_err(|_| "the dropping thread panicked")?;
    webkit::pump_run_loop(0.1);
    Ok(())
}

fn web_extension_constructors_return_results() -> TestResult {
    match WebExtensionController::new() {
        Ok(controller) => check(
            controller.configuration().is_some(),
            "controller configuration",
        )?,
        Err(WebKitError::Unsupported(_)) => {}
        Err(error) => return Err(error.into()),
    }
    match WebExtensionMatchPattern::all_urls() {
        Ok(pattern) => check(
            pattern.matches_url("https://example.test/"),
            "the all-URLs pattern matches",
        )?,
        Err(WebKitError::Unsupported(_)) => {}
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

fn webkit_is_usable() -> Result<(), String> {
    let view = WebView::with_config(&configuration()).map_err(|error| error.to_string())?;
    view.load_html("<p>probe</p>", None)
        .map_err(|error| error.to_string())?;
    let value = view
        .evaluate_javascript("1 + 1")
        .map_err(|error| error.to_string())?;
    if value == "2" {
        Ok(())
    } else {
        Err(format!("unexpected probe result {value:?}"))
    }
}

type Test = (&'static str, fn() -> TestResult);

const TESTS: &[Test] = &[
    (
        "url_scheme_task_rejects_out_of_order_calls",
        url_scheme_task_rejects_out_of_order_calls,
    ),
    (
        "url_scheme_task_fail_without_response_is_allowed_once",
        url_scheme_task_fail_without_response_is_allowed_once,
    ),
    (
        "url_scheme_task_calls_after_stop_return_errors",
        url_scheme_task_calls_after_stop_return_errors,
    ),
    (
        "url_scheme_handler_receives_request_bodies",
        url_scheme_handler_receives_request_bodies,
    ),
    (
        "script_messages_carry_frame_origin_and_world",
        script_messages_carry_frame_origin_and_world,
    ),
    (
        "isolated_world_handler_is_hidden_from_page_scripts",
        isolated_world_handler_is_hidden_from_page_scripts,
    ),
    (
        "duplicate_handler_names_are_rejected",
        duplicate_handler_names_are_rejected,
    ),
    (
        "web_views_sharing_a_configuration_get_their_own_messages",
        web_views_sharing_a_configuration_get_their_own_messages,
    ),
    (
        "reply_handler_can_reject_cross_origin_frames",
        reply_handler_can_reject_cross_origin_frames,
    ),
    (
        "navigation_action_handler_decides_each_request",
        navigation_action_handler_decides_each_request,
    ),
    (
        "navigation_response_handler_decides_each_response",
        navigation_response_handler_decides_each_response,
    ),
    (
        "call_async_javascript_uses_arguments_and_worlds",
        call_async_javascript_uses_arguments_and_worlds,
    ),
    (
        "event_queues_drop_the_oldest_entries",
        event_queues_drop_the_oldest_entries,
    ),
    (
        "handlers_swap_atomically_and_are_released",
        handlers_swap_atomically_and_are_released,
    ),
    (
        "download_file_names_cannot_escape_the_destination",
        download_file_names_cannot_escape_the_destination,
    ),
    (
        "calls_from_other_threads_run_on_the_main_thread",
        calls_from_other_threads_run_on_the_main_thread,
    ),
    (
        "web_extension_constructors_return_results",
        web_extension_constructors_return_results,
    ),
];

fn main() -> ExitCode {
    let home = match scratch_dir("home") {
        Ok(home) => home,
        Err(error) => {
            eprintln!("cannot create the scratch home directory: {error}");
            return ExitCode::FAILURE;
        }
    };
    std::env::set_var("CFFIXED_USER_HOME", &home);

    let filters: Vec<String> = std::env::args()
        .skip(1)
        .filter(|argument| !argument.starts_with('-'))
        .collect();
    let selected: Vec<_> = TESTS
        .iter()
        .filter(|(name, _)| {
            filters.is_empty() || filters.iter().any(|filter| name.contains(filter.as_str()))
        })
        .collect();

    webkit::init_app();
    if let Err(reason) = webkit_is_usable() {
        println!(
            "skipping {} main-thread WebKit tests: {reason}",
            selected.len()
        );
        println!(
            "test result: ok. 0 passed; 0 failed; {} ignored",
            selected.len()
        );
        return ExitCode::SUCCESS;
    }

    println!("running {} main-thread WebKit tests", selected.len());
    let mut failed = Vec::new();
    for (name, test) in &selected {
        let outcome = panic::catch_unwind(AssertUnwindSafe(test));
        match outcome {
            Ok(Ok(())) => println!("test {name} ... ok"),
            Ok(Err(error)) => {
                println!("test {name} ... FAILED: {error}");
                failed.push(*name);
            }
            Err(_) => {
                println!("test {name} ... FAILED: panicked");
                failed.push(*name);
            }
        }
        webkit::pump_run_loop(0.05);
    }
    println!(
        "test result: {}. {} passed; {} failed",
        if failed.is_empty() { "ok" } else { "FAILED" },
        selected.len() - failed.len(),
        failed.len()
    );
    if failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
