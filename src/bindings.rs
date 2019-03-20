/* automatically generated by rust-bindgen */

pub const NNG_MAJOR_VERSION: u32 = 1;
pub const NNG_MINOR_VERSION: u32 = 1;
pub const NNG_PATCH_VERSION: u32 = 1;
pub const NNG_RELEASE_SUFFIX: &'static [u8; 1usize] = b"\0";
pub const NNG_MAXADDRLEN: u32 = 128;
pub const NNG_DURATION_INFINITE: i32 = -1;
pub const NNG_DURATION_DEFAULT: i32 = -2;
pub const NNG_DURATION_ZERO: u32 = 0;
pub const NNG_OPT_SOCKNAME: &'static [u8; 12usize] = b"socket-name\0";
pub const NNG_OPT_RAW: &'static [u8; 4usize] = b"raw\0";
pub const NNG_OPT_PROTO: &'static [u8; 9usize] = b"protocol\0";
pub const NNG_OPT_PROTONAME: &'static [u8; 14usize] = b"protocol-name\0";
pub const NNG_OPT_PEER: &'static [u8; 5usize] = b"peer\0";
pub const NNG_OPT_PEERNAME: &'static [u8; 10usize] = b"peer-name\0";
pub const NNG_OPT_RECVBUF: &'static [u8; 12usize] = b"recv-buffer\0";
pub const NNG_OPT_SENDBUF: &'static [u8; 12usize] = b"send-buffer\0";
pub const NNG_OPT_RECVFD: &'static [u8; 8usize] = b"recv-fd\0";
pub const NNG_OPT_SENDFD: &'static [u8; 8usize] = b"send-fd\0";
pub const NNG_OPT_RECVTIMEO: &'static [u8; 13usize] = b"recv-timeout\0";
pub const NNG_OPT_SENDTIMEO: &'static [u8; 13usize] = b"send-timeout\0";
pub const NNG_OPT_LOCADDR: &'static [u8; 14usize] = b"local-address\0";
pub const NNG_OPT_REMADDR: &'static [u8; 15usize] = b"remote-address\0";
pub const NNG_OPT_URL: &'static [u8; 4usize] = b"url\0";
pub const NNG_OPT_MAXTTL: &'static [u8; 8usize] = b"ttl-max\0";
pub const NNG_OPT_RECVMAXSZ: &'static [u8; 14usize] = b"recv-size-max\0";
pub const NNG_OPT_RECONNMINT: &'static [u8; 19usize] = b"reconnect-time-min\0";
pub const NNG_OPT_RECONNMAXT: &'static [u8; 19usize] = b"reconnect-time-max\0";
pub const NNG_OPT_TLS_CONFIG: &'static [u8; 11usize] = b"tls-config\0";
pub const NNG_OPT_TLS_AUTH_MODE: &'static [u8; 13usize] = b"tls-authmode\0";
pub const NNG_OPT_TLS_CERT_KEY_FILE: &'static [u8; 18usize] = b"tls-cert-key-file\0";
pub const NNG_OPT_TLS_CA_FILE: &'static [u8; 12usize] = b"tls-ca-file\0";
pub const NNG_OPT_TLS_SERVER_NAME: &'static [u8; 16usize] = b"tls-server-name\0";
pub const NNG_OPT_TLS_VERIFIED: &'static [u8; 13usize] = b"tls-verified\0";
pub const NNG_OPT_TCP_NODELAY: &'static [u8; 12usize] = b"tcp-nodelay\0";
pub const NNG_OPT_TCP_KEEPALIVE: &'static [u8; 14usize] = b"tcp-keepalive\0";
pub const NNG_OPT_PAIR1_POLY: &'static [u8; 18usize] = b"pair1:polyamorous\0";
pub const NNG_OPT_SUB_SUBSCRIBE: &'static [u8; 14usize] = b"sub:subscribe\0";
pub const NNG_OPT_SUB_UNSUBSCRIBE: &'static [u8; 16usize] = b"sub:unsubscribe\0";
pub const NNG_OPT_REQ_RESENDTIME: &'static [u8; 16usize] = b"req:resend-time\0";
pub const NNG_OPT_SURVEYOR_SURVEYTIME: &'static [u8; 21usize] = b"surveyor:survey-time\0";
pub const NNG_OPT_IPC_SECURITY_DESCRIPTOR: &'static [u8; 24usize] = b"ipc:security-descriptor\0";
pub const NNG_OPT_IPC_PERMISSIONS: &'static [u8; 16usize] = b"ipc:permissions\0";
pub const NNG_OPT_IPC_PEER_UID: &'static [u8; 13usize] = b"ipc:peer-uid\0";
pub const NNG_OPT_IPC_PEER_GID: &'static [u8; 13usize] = b"ipc:peer-gid\0";
pub const NNG_OPT_IPC_PEER_PID: &'static [u8; 13usize] = b"ipc:peer-pid\0";
pub const NNG_OPT_IPC_PEER_ZONEID: &'static [u8; 16usize] = b"ipc:peer-zoneid\0";
pub const NNG_OPT_WS_REQUEST_HEADERS: &'static [u8; 19usize] = b"ws:request-headers\0";
pub const NNG_OPT_WS_RESPONSE_HEADERS: &'static [u8; 20usize] = b"ws:response-headers\0";
pub const NNG_OPT_WSS_REQUEST_HEADERS: &'static [u8; 19usize] = b"ws:request-headers\0";
pub const NNG_OPT_WSS_RESPONSE_HEADERS: &'static [u8; 20usize] = b"ws:response-headers\0";
pub const NNG_OPT_ZT_HOME: &'static [u8; 8usize] = b"zt:home\0";
pub const NNG_OPT_ZT_NWID: &'static [u8; 8usize] = b"zt:nwid\0";
pub const NNG_OPT_ZT_NODE: &'static [u8; 8usize] = b"zt:node\0";
pub const NNG_OPT_ZT_NETWORK_STATUS: &'static [u8; 18usize] = b"zt:network-status\0";
pub const NNG_OPT_ZT_NETWORK_NAME: &'static [u8; 16usize] = b"zt:network-name\0";
pub const NNG_OPT_ZT_PING_TIME: &'static [u8; 13usize] = b"zt:ping-time\0";
pub const NNG_OPT_ZT_PING_TRIES: &'static [u8; 14usize] = b"zt:ping-tries\0";
pub const NNG_OPT_ZT_CONN_TIME: &'static [u8; 13usize] = b"zt:conn-time\0";
pub const NNG_OPT_ZT_CONN_TRIES: &'static [u8; 14usize] = b"zt:conn-tries\0";
pub const NNG_OPT_ZT_MTU: &'static [u8; 7usize] = b"zt:mtu\0";
pub const NNG_OPT_ZT_ORBIT: &'static [u8; 9usize] = b"zt:orbit\0";
pub const NNG_OPT_ZT_DEORBIT: &'static [u8; 11usize] = b"zt:deorbit\0";
pub const NNG_OPT_ZT_ADD_LOCAL_ADDR: &'static [u8; 18usize] = b"zt:add-local-addr\0";
pub const NNG_OPT_ZT_CLEAR_LOCAL_ADDRS: &'static [u8; 21usize] = b"zt:clear-local-addrs\0";
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __uint64_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_ctx_s {
    pub _bindgen_opaque_blob: u32,
}
pub type nng_ctx = nng_ctx_s;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_dialer_s {
    pub _bindgen_opaque_blob: u32,
}
pub type nng_dialer = nng_dialer_s;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_listener_s {
    pub _bindgen_opaque_blob: u32,
}
pub type nng_listener = nng_listener_s;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_pipe_s {
    pub _bindgen_opaque_blob: u32,
}
pub type nng_pipe = nng_pipe_s;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_socket_s {
    pub _bindgen_opaque_blob: u32,
}
pub type nng_socket = nng_socket_s;
pub type nng_duration = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_msg {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_stat {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_aio {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nng_sockaddr_inproc {
    pub sa_family: u16,
    pub sa_name: [::std::os::raw::c_char; 128usize],
}
impl Default for nng_sockaddr_inproc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nng_sockaddr_path {
    pub sa_family: u16,
    pub sa_path: [::std::os::raw::c_char; 128usize],
}
impl Default for nng_sockaddr_path {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type nng_sockaddr_ipc = nng_sockaddr_path;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_sockaddr_in6 {
    pub sa_family: u16,
    pub sa_port: u16,
    pub sa_addr: [u8; 16usize],
}
pub type nng_sockaddr_udp6 = nng_sockaddr_in6;
pub type nng_sockaddr_tcp6 = nng_sockaddr_in6;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_sockaddr_in {
    pub sa_family: u16,
    pub sa_port: u16,
    pub sa_addr: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nng_sockaddr_zt {
    pub sa_family: u16,
    pub sa_nwid: u64,
    pub sa_nodeid: u64,
    pub sa_port: u32,
}
pub type nng_sockaddr_udp = nng_sockaddr_in;
pub type nng_sockaddr_tcp = nng_sockaddr_in;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nng_sockaddr {
    pub s_family: u16,
    pub s_ipc: nng_sockaddr_ipc,
    pub s_inproc: nng_sockaddr_inproc,
    pub s_in6: nng_sockaddr_in6,
    pub s_in: nng_sockaddr_in,
    pub s_zt: nng_sockaddr_zt,
    _bindgen_union_align: [u64; 17usize],
}
impl Default for nng_sockaddr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_sockaddr_family {
    NNG_AF_UNSPEC = 0,
    NNG_AF_INPROC = 1,
    NNG_AF_IPC = 2,
    NNG_AF_INET = 3,
    NNG_AF_INET6 = 4,
    NNG_AF_ZT = 5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_iov {
    pub iov_buf: *mut ::std::os::raw::c_void,
    pub iov_len: usize,
}
impl Default for nng_iov {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
extern "C" {
    pub fn nng_fini();
}
extern "C" {
    pub fn nng_close(arg1: nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_socket_id(arg1: nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_closeall();
}
extern "C" {
    pub fn nng_setopt(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_void,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_setopt_bool(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_setopt_int(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_setopt_ms(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_setopt_size(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_setopt_uint64(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_setopt_string(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_setopt_ptr(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt_bool(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt_int(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt_ms(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt_size(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt_uint64(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt_ptr(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_pipe_ev {
    NNG_PIPE_EV_ADD_PRE = 0,
    NNG_PIPE_EV_ADD_POST = 1,
    NNG_PIPE_EV_REM_POST = 2,
}
pub type nng_pipe_cb = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: nng_pipe,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn nng_pipe_notify(
        arg1: nng_socket,
        arg2: ::std::os::raw::c_int,
        arg3: nng_pipe_cb,
        arg4: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_getopt_string(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listen(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_listener,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dial(
        arg1: nng_socket,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_dialer,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_create(
        arg1: *mut nng_dialer,
        arg2: nng_socket,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_create(
        arg1: *mut nng_listener,
        arg2: nng_socket,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_start(arg1: nng_dialer, arg2: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_start(
        arg1: nng_listener,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_close(arg1: nng_dialer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_close(arg1: nng_listener) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_id(arg1: nng_dialer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_id(arg1: nng_listener) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_void,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt_bool(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt_int(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt_ms(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt_size(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt_uint64(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt_ptr(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_setopt_string(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_bool(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_int(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_ms(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_size(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_sockaddr(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_sockaddr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_uint64(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_ptr(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_dialer_getopt_string(
        arg1: nng_dialer,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_void,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt_bool(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt_int(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt_ms(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt_size(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt_uint64(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt_ptr(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_setopt_string(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_bool(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_int(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_ms(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_size(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_sockaddr(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_sockaddr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_uint64(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_ptr(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_listener_getopt_string(
        arg1: nng_listener,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_strerror(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn nng_send(
        arg1: nng_socket,
        arg2: *mut ::std::os::raw::c_void,
        arg3: usize,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_recv(
        arg1: nng_socket,
        arg2: *mut ::std::os::raw::c_void,
        arg3: *mut usize,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_sendmsg(
        arg1: nng_socket,
        arg2: *mut nng_msg,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_recvmsg(
        arg1: nng_socket,
        arg2: *mut *mut nng_msg,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_send_aio(arg1: nng_socket, arg2: *mut nng_aio);
}
extern "C" {
    pub fn nng_recv_aio(arg1: nng_socket, arg2: *mut nng_aio);
}
extern "C" {
    pub fn nng_ctx_open(arg1: *mut nng_ctx, arg2: nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_close(arg1: nng_ctx) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_id(arg1: nng_ctx) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_recv(arg1: nng_ctx, arg2: *mut nng_aio);
}
extern "C" {
    pub fn nng_ctx_send(arg1: nng_ctx, arg2: *mut nng_aio);
}
extern "C" {
    pub fn nng_ctx_getopt(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_getopt_bool(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_getopt_int(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_getopt_ms(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_getopt_size(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_setopt(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_void,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_setopt_bool(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_setopt_int(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_setopt_ms(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ctx_setopt_size(
        arg1: nng_ctx,
        arg2: *const ::std::os::raw::c_char,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_alloc(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn nng_free(arg1: *mut ::std::os::raw::c_void, arg2: usize);
}
extern "C" {
    pub fn nng_strdup(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn nng_strfree(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn nng_aio_alloc(
        arg1: *mut *mut nng_aio,
        arg2: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_aio_free(arg1: *mut nng_aio);
}
extern "C" {
    pub fn nng_aio_stop(arg1: *mut nng_aio);
}
extern "C" {
    pub fn nng_aio_result(arg1: *mut nng_aio) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_aio_count(arg1: *mut nng_aio) -> usize;
}
extern "C" {
    pub fn nng_aio_cancel(arg1: *mut nng_aio);
}
extern "C" {
    pub fn nng_aio_abort(arg1: *mut nng_aio, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn nng_aio_wait(arg1: *mut nng_aio);
}
extern "C" {
    pub fn nng_aio_set_msg(arg1: *mut nng_aio, arg2: *mut nng_msg);
}
extern "C" {
    pub fn nng_aio_get_msg(arg1: *mut nng_aio) -> *mut nng_msg;
}
extern "C" {
    pub fn nng_aio_set_input(
        arg1: *mut nng_aio,
        arg2: ::std::os::raw::c_uint,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_aio_get_input(
        arg1: *mut nng_aio,
        arg2: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn nng_aio_set_output(
        arg1: *mut nng_aio,
        arg2: ::std::os::raw::c_uint,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_aio_get_output(
        arg1: *mut nng_aio,
        arg2: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn nng_aio_set_timeout(arg1: *mut nng_aio, arg2: nng_duration);
}
extern "C" {
    pub fn nng_aio_set_iov(
        arg1: *mut nng_aio,
        arg2: ::std::os::raw::c_uint,
        arg3: *const nng_iov,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_aio_begin(arg1: *mut nng_aio) -> bool;
}
extern "C" {
    pub fn nng_aio_finish(arg1: *mut nng_aio, arg2: ::std::os::raw::c_int);
}
pub type nng_aio_cancelfn = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut nng_aio,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
    ),
>;
extern "C" {
    pub fn nng_aio_defer(
        arg1: *mut nng_aio,
        arg2: nng_aio_cancelfn,
        arg3: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn nng_sleep_aio(arg1: nng_duration, arg2: *mut nng_aio);
}
extern "C" {
    pub fn nng_msg_alloc(arg1: *mut *mut nng_msg, arg2: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_free(arg1: *mut nng_msg);
}
extern "C" {
    pub fn nng_msg_realloc(arg1: *mut nng_msg, arg2: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header(arg1: *mut nng_msg) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn nng_msg_header_len(arg1: *const nng_msg) -> usize;
}
extern "C" {
    pub fn nng_msg_body(arg1: *mut nng_msg) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn nng_msg_len(arg1: *const nng_msg) -> usize;
}
extern "C" {
    pub fn nng_msg_append(
        arg1: *mut nng_msg,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_insert(
        arg1: *mut nng_msg,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_trim(arg1: *mut nng_msg, arg2: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_chop(arg1: *mut nng_msg, arg2: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_append(
        arg1: *mut nng_msg,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_insert(
        arg1: *mut nng_msg,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_trim(arg1: *mut nng_msg, arg2: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_chop(arg1: *mut nng_msg, arg2: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_append_u16(arg1: *mut nng_msg, arg2: u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_append_u32(arg1: *mut nng_msg, arg2: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_append_u64(arg1: *mut nng_msg, arg2: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_insert_u16(arg1: *mut nng_msg, arg2: u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_insert_u32(arg1: *mut nng_msg, arg2: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_insert_u64(arg1: *mut nng_msg, arg2: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_chop_u16(arg1: *mut nng_msg, arg2: *mut u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_chop_u32(arg1: *mut nng_msg, arg2: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_chop_u64(arg1: *mut nng_msg, arg2: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_trim_u16(arg1: *mut nng_msg, arg2: *mut u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_trim_u32(arg1: *mut nng_msg, arg2: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_header_trim_u64(arg1: *mut nng_msg, arg2: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_append_u16(arg1: *mut nng_msg, arg2: u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_append_u32(arg1: *mut nng_msg, arg2: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_append_u64(arg1: *mut nng_msg, arg2: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_insert_u16(arg1: *mut nng_msg, arg2: u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_insert_u32(arg1: *mut nng_msg, arg2: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_insert_u64(arg1: *mut nng_msg, arg2: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_chop_u16(arg1: *mut nng_msg, arg2: *mut u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_chop_u32(arg1: *mut nng_msg, arg2: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_chop_u64(arg1: *mut nng_msg, arg2: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_trim_u16(arg1: *mut nng_msg, arg2: *mut u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_trim_u32(arg1: *mut nng_msg, arg2: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_trim_u64(arg1: *mut nng_msg, arg2: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_dup(arg1: *mut *mut nng_msg, arg2: *const nng_msg) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_msg_clear(arg1: *mut nng_msg);
}
extern "C" {
    pub fn nng_msg_header_clear(arg1: *mut nng_msg);
}
extern "C" {
    pub fn nng_msg_set_pipe(arg1: *mut nng_msg, arg2: nng_pipe);
}
extern "C" {
    pub fn nng_msg_get_pipe(arg1: *const nng_msg) -> nng_pipe;
}
extern "C" {
    pub fn nng_msg_getopt(
        arg1: *mut nng_msg,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_bool(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_int(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_ms(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_duration,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_size(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_sockaddr(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nng_sockaddr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_uint64(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_ptr(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_getopt_string(
        arg1: nng_pipe,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_close(arg1: nng_pipe) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_id(arg1: nng_pipe) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pipe_socket(arg1: nng_pipe) -> nng_socket;
}
extern "C" {
    pub fn nng_pipe_dialer(arg1: nng_pipe) -> nng_dialer;
}
extern "C" {
    pub fn nng_pipe_listener(arg1: nng_pipe) -> nng_listener;
}
pub const NNG_FLAG_ALLOC: nng_flag_enum = 1;
pub const NNG_FLAG_NONBLOCK: nng_flag_enum = 2;
pub type nng_flag_enum = u32;
extern "C" {
    pub fn nng_stats_get(arg1: *mut *mut nng_stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_stats_free(arg1: *mut nng_stat);
}
extern "C" {
    pub fn nng_stats_dump(arg1: *mut nng_stat);
}
extern "C" {
    pub fn nng_stat_next(arg1: *mut nng_stat) -> *mut nng_stat;
}
extern "C" {
    pub fn nng_stat_child(arg1: *mut nng_stat) -> *mut nng_stat;
}
extern "C" {
    pub fn nng_stat_name(arg1: *mut nng_stat) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn nng_stat_type(arg1: *mut nng_stat) -> ::std::os::raw::c_int;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_stat_type_enum {
    NNG_STAT_SCOPE = 0,
    NNG_STAT_LEVEL = 1,
    NNG_STAT_COUNTER = 2,
    NNG_STAT_STRING = 3,
    NNG_STAT_BOOLEAN = 4,
    NNG_STAT_ID = 5,
}
extern "C" {
    pub fn nng_stat_unit(arg1: *mut nng_stat) -> ::std::os::raw::c_int;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_unit_enum {
    NNG_UNIT_NONE = 0,
    NNG_UNIT_BYTES = 1,
    NNG_UNIT_MESSAGES = 2,
    NNG_UNIT_MILLIS = 3,
    NNG_UNIT_EVENTS = 4,
}
extern "C" {
    pub fn nng_stat_value(arg1: *mut nng_stat) -> u64;
}
extern "C" {
    pub fn nng_stat_string(arg1: *mut nng_stat) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn nng_stat_desc(arg1: *mut nng_stat) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn nng_stat_timestamp(arg1: *mut nng_stat) -> u64;
}
extern "C" {
    pub fn nng_device(arg1: nng_socket, arg2: nng_socket) -> ::std::os::raw::c_int;
}
pub const NNG_EINTR: nng_errno_enum = 1;
pub const NNG_ENOMEM: nng_errno_enum = 2;
pub const NNG_EINVAL: nng_errno_enum = 3;
pub const NNG_EBUSY: nng_errno_enum = 4;
pub const NNG_ETIMEDOUT: nng_errno_enum = 5;
pub const NNG_ECONNREFUSED: nng_errno_enum = 6;
pub const NNG_ECLOSED: nng_errno_enum = 7;
pub const NNG_EAGAIN: nng_errno_enum = 8;
pub const NNG_ENOTSUP: nng_errno_enum = 9;
pub const NNG_EADDRINUSE: nng_errno_enum = 10;
pub const NNG_ESTATE: nng_errno_enum = 11;
pub const NNG_ENOENT: nng_errno_enum = 12;
pub const NNG_EPROTO: nng_errno_enum = 13;
pub const NNG_EUNREACHABLE: nng_errno_enum = 14;
pub const NNG_EADDRINVAL: nng_errno_enum = 15;
pub const NNG_EPERM: nng_errno_enum = 16;
pub const NNG_EMSGSIZE: nng_errno_enum = 17;
pub const NNG_ECONNABORTED: nng_errno_enum = 18;
pub const NNG_ECONNRESET: nng_errno_enum = 19;
pub const NNG_ECANCELED: nng_errno_enum = 20;
pub const NNG_ENOFILES: nng_errno_enum = 21;
pub const NNG_ENOSPC: nng_errno_enum = 22;
pub const NNG_EEXIST: nng_errno_enum = 23;
pub const NNG_EREADONLY: nng_errno_enum = 24;
pub const NNG_EWRITEONLY: nng_errno_enum = 25;
pub const NNG_ECRYPTO: nng_errno_enum = 26;
pub const NNG_EPEERAUTH: nng_errno_enum = 27;
pub const NNG_ENOARG: nng_errno_enum = 28;
pub const NNG_EAMBIGUOUS: nng_errno_enum = 29;
pub const NNG_EBADTYPE: nng_errno_enum = 30;
pub const NNG_EINTERNAL: nng_errno_enum = 1000;
pub const NNG_ESYSERR: nng_errno_enum = 268435456;
pub const NNG_ETRANERR: nng_errno_enum = 536870912;
pub type nng_errno_enum = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_url {
    pub u_rawurl: *mut ::std::os::raw::c_char,
    pub u_scheme: *mut ::std::os::raw::c_char,
    pub u_userinfo: *mut ::std::os::raw::c_char,
    pub u_host: *mut ::std::os::raw::c_char,
    pub u_hostname: *mut ::std::os::raw::c_char,
    pub u_port: *mut ::std::os::raw::c_char,
    pub u_path: *mut ::std::os::raw::c_char,
    pub u_query: *mut ::std::os::raw::c_char,
    pub u_fragment: *mut ::std::os::raw::c_char,
    pub u_requri: *mut ::std::os::raw::c_char,
}
impl Default for nng_url {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
extern "C" {
    pub fn nng_url_parse(
        arg1: *mut *mut nng_url,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_url_free(arg1: *mut nng_url);
}
extern "C" {
    pub fn nng_url_clone(arg1: *mut *mut nng_url, arg2: *const nng_url) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn nng_bus0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_bus0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pair0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pair0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pair1_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pair1_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pull0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pull0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_push0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_push0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pub0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_pub0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_sub0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_sub0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_rep0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_rep0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_req0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_req0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_respondent0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_respondent0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_surveyor0_open(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_surveyor0_open_raw(arg1: *mut nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_inproc_register() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ipc_register() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_tcp_register() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_tls_register() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_ws_register() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nng_wss_register() -> ::std::os::raw::c_int;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_zt_status {
    NNG_ZT_STATUS_UP = 0,
    NNG_ZT_STATUS_CONFIG = 1,
    NNG_ZT_STATUS_DENIED = 2,
    NNG_ZT_STATUS_NOTFOUND = 3,
    NNG_ZT_STATUS_ERROR = 4,
    NNG_ZT_STATUS_OBSOLETE = 5,
    NNG_ZT_STATUS_UNKNOWN = 6,
}
extern "C" {
    pub fn nng_zt_register() -> ::std::os::raw::c_int;
}
