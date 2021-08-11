mod socket;

// IMPORTANT NOTE:
// Most of the tests involving the "all" feature will fail because some functions and constants of libc
// do not have an equivalent in sgx_libc.

pub fn test_socket2() {
    println!("Testing socket2 inside enclave.");

    print!("Testing domain_for_address()...");
    socket::domain_for_address();
    print!("Success.\n");

    print!("Testing domain_fmt_debug()...");
    socket::domain_fmt_debug();
    print!("Success.\n");

    print!("Testing type_fmt_debug()...");
    socket::type_fmt_debug();
    print!("Success.\n");

    print!("Testing protocol_fmt_debug()...");
    socket::protocol_fmt_debug();
    print!("Success.\n");

    //Insert should_panic test
    #[cfg(all(any(unix, target_env = "sgx"), feature = "all"))]
    {
        print!("Testing socket_address_unix()...");
        socket::socket_address_unix();
        print!("Success.\n");
    }

    #[cfg(all(any(target_os = "linux", target_os = "android", target_env = "sgx"), feature = "all"))]
    {
        print!("Testing socket_address_unix_abstract_namespace()...");
        socket::socket_address_unix_abstract_namespace();
        print!("Success.\n");
    }

    #[cfg(all(feature = "all", any(target_os = "android", target_os = "linux", target_env = "sgx")))]
    {
        print!("Testing socket_address_vsock()...");
        socket::socket_address_vsock();
        print!("Success.\n");
    }

    print!("Testing set_nonblocking()...");
    socket::set_nonblocking();
    print!("Success.\n");

    print!("Testing common_flags()...");
    socket::common_flags();
    print!("Success.\n");
    
    print!("Testing common_flags()...");
    socket::no_common_flags();
    print!("Success.\n");

    #[cfg(all(
        feature = "all",
        any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd",
            target_env = "sgx"
        )
    ))]
    {
        print!("Testing type_nonblocking()...");
        socket::type_nonblocking();
        print!("Success.\n");
    }

    #[cfg(all(any(unix, target_env = "sgx"), feature = "all"))]
    {
        print!("Testing set_cloexec()...");
        socket::set_cloexec();
        print!("Success.\n");
    }

    #[cfg(all(
        feature = "all",
        any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd",
            target_env = "sgx"
        )
    ))]
    {
        print!("Testing type_cloexec()...");
        socket::type_cloexec();
        print!("Success.\n");
    }

    print!("Testing connect_timeout_unrouteable()...");
    socket::connect_timeout_unrouteable();
    print!("Success.\n");

    print!("Testing connect_timeout_unbound()...");
    socket::connect_timeout_unbound();
    print!("Success.\n");

    print!("Testing connect_timeout_valid()...");
    socket::connect_timeout_valid();
    print!("Success.\n");

    #[cfg(all(feature = "all", any(unix, target_env = "sgx")))]
    {
        print!("Testing pair()...");
        socket::pair();
        print!("Success.\n");
    }

    #[cfg(all(feature = "all", any(unix, target_env = "sgx")))]
    {
        print!("Testing unix()...");
        socket::unix();
        print!("Success.\n");
    }

    print!("Testing out_of_band()...");
    socket::out_of_band();
    print!("Success.\n");

    /*#[cfg(not(target_os = "redox"))]
    {
        print!("Testing send_recv_vectored()...");
        socket::send_recv_vectored();
        print!("Success.\n");
    }*/
    //This test did not pass in the original crate.

    /*#[cfg(not(target_os = "redox"))]
    {
        print!("Testing send_from_recv_to_vectored()...");
        socket::send_from_recv_to_vectored();
        print!("Success.\n");
    }*/
    //This test did not pass in the original crate.

    /*#[cfg(not(target_os = "redox"))]
    {
        print!("Testing recv_vectored_truncated()...");
        socket::recv_vectored_truncated();
        print!("Success.\n");
    }*/
    //This test did not pass in the original crate.

    /*#[cfg(not(target_os = "redox"))]
    {
        print!("Testing recv_from_vectored_truncated()...");
        socket::recv_from_vectored_truncated();
        print!("Success.\n");
    }*/
    //This test did not pass in the original crate.

    print!("Testing tcp_keepalive()...");
    socket::tcp_keepalive();
    print!("Success.\n");

    #[cfg(all(
        feature = "all",
        any(
            target_os = "android",
            target_os = "freebsd",
            target_os = "linux",
            target_vendor = "apple",
            target_env = "sgx"
        )
    ))]
    {
        print!("Testing sendfile()...");
        socket::sendfile();
        print!("Success.\n");
    }

    #[cfg(all(
        feature = "all",
        any(
            target_os = "android",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "linux",
            target_env = "sgx"
        )
    ))]
    {
        print!("Testing is_listener()...");
        socket::is_listener();
        print!("Success.\n");
    }

    #[cfg(all(
        feature = "all",
        any(
            target_os = "android",
            // TODO: add FreeBSD.
            // target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "linux",
            target_env = "sgx"
        )
    ))]
    {
        print!("Testing domain()...");
        socket::domain();
        print!("Success.\n");
    }

    #[cfg(all(
        feature = "all",
        any(
            target_os = "android",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "linux",
            target_env = "sgx"
        )
    ))]
    {
        print!("Testing protocol()...");
        socket::protocol();
        print!("Success.\n");
    }

    print!("Testing r#type()...");
    socket::r#type();
    print!("Success.\n");

    #[cfg(all(feature = "all", any(target_os = "linux", target_env = "sgx")))]
    {
        print!("Testing cpu_affinity()...");
        socket::cpu_affinity();
        print!("Success.\n");
    }

    print!("Testing niche()...");
    socket::niche();
    print!("Success.\n");

    print!("Testing nodelay()...");
    socket::nodelay();
    print!("Success.\n");

    print!("Testing recv_buffer_size()...");
    socket::recv_buffer_size();
    print!("Success.\n");

    print!("Testing send_buffer_size()...");
    socket::send_buffer_size();
    print!("Success.\n");

    print!("Testing out_of_band_inline()...");
    socket::out_of_band_inline();
    print!("Success.\n");

    print!("Testing reuse_address()...");
    socket::reuse_address();
    print!("Success.\n");

    print!("Testing linger()...");
    socket::linger();
    print!("Success.\n");

    print!("Testing read_timeout()...");
    socket::read_timeout();
    print!("Success.\n");

    print!("Testing keepalive()...");
    socket::keepalive();
    print!("Success.\n");

    print!("Testing ttl()...");
    socket::ttl();
    print!("Success.\n");

    print!("Testing tos()...");
    socket::tos();
    print!("Success.\n");

    print!("Testing broadcast()...");
    socket::broadcast();
    print!("Success.\n");

    print!("Testing unicast_hops_v6()...");
    socket::unicast_hops_v6();
    print!("Success.\n");

    print!("Testing only_v6()...");
    socket::only_v6();
    print!("Success.\n");

}