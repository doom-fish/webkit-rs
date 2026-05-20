import Foundation
import Network

@available(macOS 14.0, *)
final class WKProxyConfigurationBox: NSObject {
    var proxyConfiguration: Network.ProxyConfiguration

    init(proxyConfiguration: Network.ProxyConfiguration) {
        self.proxyConfiguration = proxyConfiguration
        super.init()
    }
}

@available(macOS 14.0, *)
private func wkProxyConfigurationSummary(
    _ proxyConfiguration: Network.ProxyConfiguration
) -> [String: Any] {
    [
        "description": proxyConfiguration.debugDescription,
        "failoverAllowed": proxyConfiguration.allowFailover,
        "matchDomains": proxyConfiguration.matchDomains,
        "excludedDomains": proxyConfiguration.excludedDomains
    ]
}

private func wkMakeProxyEndpoint(
    _ host: UnsafePointer<CChar>?,
    _ port: UInt16,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> NWEndpoint? {
    guard let host else {
        outErr?.pointee = wkCString("missing proxy host")
        return nil
    }

    let hostString = String(cString: host)
    guard !hostString.isEmpty else {
        outErr?.pointee = wkCString("proxy host must not be empty")
        return nil
    }
    guard let proxyPort = try? NWEndpoint.Port(rawValue: port) else {
        outErr?.pointee = wkCString("invalid proxy port")
        return nil
    }

    return .hostPort(host: NWEndpoint.Host(hostString), port: proxyPort)
}

@_cdecl("wk_proxy_configuration_create_http_connect")
public func wk_proxy_configuration_create_http_connect(
    _ host: UnsafePointer<CChar>?,
    _ port: UInt16,
    _ outProxyConfiguration: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("HTTP CONNECT proxy configurations require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    guard let endpoint = wkMakeProxyEndpoint(host, port, outErr) else {
        return WK_INVALID_ARGUMENT
    }
    let proxyConfiguration = Network.ProxyConfiguration(httpCONNECTProxy: endpoint)
    outProxyConfiguration?.pointee = wkRetain(
        WKProxyConfigurationBox(proxyConfiguration: proxyConfiguration)
    )
    return WK_OK
}

@_cdecl("wk_proxy_configuration_create_socksv5")
public func wk_proxy_configuration_create_socksv5(
    _ host: UnsafePointer<CChar>?,
    _ port: UInt16,
    _ outProxyConfiguration: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("SOCKSv5 proxy configurations require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    guard let endpoint = wkMakeProxyEndpoint(host, port, outErr) else {
        return WK_INVALID_ARGUMENT
    }
    let proxyConfiguration = Network.ProxyConfiguration(socksv5Proxy: endpoint)
    outProxyConfiguration?.pointee = wkRetain(
        WKProxyConfigurationBox(proxyConfiguration: proxyConfiguration)
    )
    return WK_OK
}

@_cdecl("wk_proxy_configuration_release")
public func wk_proxy_configuration_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_proxy_configuration_copy_summary_json")
public func wk_proxy_configuration_copy_summary_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing proxy configuration")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("proxy configuration summaries require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKProxyConfigurationBox = wkBorrow(ptr)
    outJson?.pointee = wkCString(wkJSONString(wkProxyConfigurationSummary(box.proxyConfiguration)))
    return WK_OK
}

@_cdecl("wk_proxy_configuration_set_username_and_password")
public func wk_proxy_configuration_set_username_and_password(
    _ ptr: UnsafeMutableRawPointer?,
    _ username: UnsafePointer<CChar>?,
    _ password: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let username else {
        outErr?.pointee = wkCString("missing proxy configuration or username")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("proxy credentials require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKProxyConfigurationBox = wkBorrow(ptr)
    nw_proxy_config_set_username_and_password(box.proxyConfiguration._nw, username, password)
    return WK_OK
}

@_cdecl("wk_proxy_configuration_set_failover_allowed")
public func wk_proxy_configuration_set_failover_allowed(
    _ ptr: UnsafeMutableRawPointer?,
    _ allowed: Bool,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing proxy configuration")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("proxy failover requires macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKProxyConfigurationBox = wkBorrow(ptr)
    box.proxyConfiguration.allowFailover = allowed
    return WK_OK
}

@_cdecl("wk_proxy_configuration_add_match_domain")
public func wk_proxy_configuration_add_match_domain(
    _ ptr: UnsafeMutableRawPointer?,
    _ domain: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let domain else {
        outErr?.pointee = wkCString("missing proxy configuration or domain")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("proxy match domains require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKProxyConfigurationBox = wkBorrow(ptr)
    box.proxyConfiguration.matchDomains.append(String(cString: domain))
    return WK_OK
}

@_cdecl("wk_proxy_configuration_clear_match_domains")
public func wk_proxy_configuration_clear_match_domains(
    _ ptr: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing proxy configuration")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("proxy match domains require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKProxyConfigurationBox = wkBorrow(ptr)
    box.proxyConfiguration.matchDomains = []
    return WK_OK
}

@_cdecl("wk_proxy_configuration_add_excluded_domain")
public func wk_proxy_configuration_add_excluded_domain(
    _ ptr: UnsafeMutableRawPointer?,
    _ domain: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let domain else {
        outErr?.pointee = wkCString("missing proxy configuration or domain")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("proxy excluded domains require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKProxyConfigurationBox = wkBorrow(ptr)
    box.proxyConfiguration.excludedDomains.append(String(cString: domain))
    return WK_OK
}

@_cdecl("wk_proxy_configuration_clear_excluded_domains")
public func wk_proxy_configuration_clear_excluded_domains(
    _ ptr: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing proxy configuration")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("proxy excluded domains require macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKProxyConfigurationBox = wkBorrow(ptr)
    box.proxyConfiguration.excludedDomains = []
    return WK_OK
}
