/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct Struct_zmq_msg_t {
    pub unknown: [c_uchar; 64usize],
}
impl ::std::clone::Clone for Struct_zmq_msg_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_zmq_msg_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type zmq_msg_t = Struct_zmq_msg_t;
pub type zmq_free_fn =
    unsafe extern "C" fn(data: *mut c_void,
                         hint: *mut c_void);
#[repr(C)]
#[derive(Copy)]
pub struct Struct_zmq_pollitem_t {
    pub socket: *mut c_void,
    #[cfg(target_os = "windows")]       // Need to fix this manually
    pub fd: ::libc::intptr_t,           // Windows Socket is defined as UINT_PTR
    #[cfg(not(target_os = "windows"))]
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}
impl ::std::clone::Clone for Struct_zmq_pollitem_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_zmq_pollitem_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type zmq_pollitem_t = Struct_zmq_pollitem_t;
pub enum Struct_iovec { }
pub type zmq_thread_fn =
    unsafe extern "C" fn(arg1: *mut c_void);
extern "C" {
    pub fn zmq_errno() -> c_int;
    pub fn zmq_strerror(errnum: c_int)
     -> *const c_char;
    pub fn zmq_version(major: *mut c_int,
                       minor: *mut c_int,
                       patch: *mut c_int);
    pub fn zmq_ctx_new() -> *mut c_void;
    pub fn zmq_ctx_term(context: *mut c_void)
     -> c_int;
    pub fn zmq_ctx_shutdown(ctx_: *mut c_void)
     -> c_int;
    pub fn zmq_ctx_set(context: *mut c_void,
                       option: c_int,
                       optval: c_int)
     -> c_int;
    pub fn zmq_ctx_get(context: *mut c_void,
                       option: c_int)
     -> c_int;
    pub fn zmq_init(io_threads: c_int)
     -> *mut c_void;
    pub fn zmq_term(context: *mut c_void)
     -> c_int;
    pub fn zmq_ctx_destroy(context: *mut c_void)
     -> c_int;
    pub fn zmq_msg_init(msg: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_init_size(msg: *mut zmq_msg_t, size: size_t)
     -> c_int;
    pub fn zmq_msg_init_data(msg: *mut zmq_msg_t,
                             data: *mut c_void, size: size_t,
                             ffn: zmq_free_fn,
                             hint: *mut c_void)
     -> c_int;
    pub fn zmq_msg_send(msg: *mut zmq_msg_t, s: *mut c_void,
                        flags: c_int)
     -> c_int;
    pub fn zmq_msg_recv(msg: *mut zmq_msg_t, s: *mut c_void,
                        flags: c_int)
     -> c_int;
    pub fn zmq_msg_close(msg: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_move(dest: *mut zmq_msg_t, src: *mut zmq_msg_t)
     -> c_int;
    pub fn zmq_msg_copy(dest: *mut zmq_msg_t, src: *mut zmq_msg_t)
     -> c_int;
    pub fn zmq_msg_data(msg: *mut zmq_msg_t) -> *mut c_void;
    pub fn zmq_msg_size(msg: *mut zmq_msg_t) -> size_t;
    pub fn zmq_msg_more(msg: *mut zmq_msg_t) -> c_int;
    pub fn zmq_msg_get(msg: *mut zmq_msg_t, property: c_int)
     -> c_int;
    pub fn zmq_msg_set(msg: *mut zmq_msg_t, property: c_int,
                       optval: c_int)
     -> c_int;
    pub fn zmq_msg_gets(msg: *mut zmq_msg_t,
                        property: *const c_char)
     -> *const c_char;
    pub fn zmq_socket(arg1: *mut c_void,
                      _type: c_int)
     -> *mut c_void;
    pub fn zmq_close(s: *mut c_void) -> c_int;
    pub fn zmq_setsockopt(s: *mut c_void,
                          option: c_int,
                          optval: *const c_void,
                          optvallen: size_t) -> c_int;
    pub fn zmq_getsockopt(s: *mut c_void,
                          option: c_int,
                          optval: *mut c_void,
                          optvallen: *mut size_t) -> c_int;
    pub fn zmq_bind(s: *mut c_void,
                    addr: *const c_char)
     -> c_int;
    pub fn zmq_connect(s: *mut c_void,
                       addr: *const c_char)
     -> c_int;
    pub fn zmq_unbind(s: *mut c_void,
                      addr: *const c_char)
     -> c_int;
    pub fn zmq_disconnect(s: *mut c_void,
                          addr: *const c_char)
     -> c_int;
    pub fn zmq_send(s: *mut c_void,
                    buf: *const c_void, len: size_t,
                    flags: c_int) -> c_int;
    pub fn zmq_send_const(s: *mut c_void,
                          buf: *const c_void, len: size_t,
                          flags: c_int)
     -> c_int;
    pub fn zmq_recv(s: *mut c_void,
                    buf: *mut c_void, len: size_t,
                    flags: c_int) -> c_int;
    pub fn zmq_socket_monitor(s: *mut c_void,
                              addr: *const c_char,
                              events: c_int)
     -> c_int;
    pub fn zmq_poll(items: *mut zmq_pollitem_t, nitems: c_int,
                    timeout: c_long) -> c_int;
    pub fn zmq_proxy(frontend: *mut c_void,
                     backend: *mut c_void,
                     capture: *mut c_void)
     -> c_int;
    pub fn zmq_proxy_steerable(frontend: *mut c_void,
                               backend: *mut c_void,
                               capture: *mut c_void,
                               control: *mut c_void)
     -> c_int;
    pub fn zmq_has(capability: *const c_char)
     -> c_int;
    pub fn zmq_device(_type: c_int,
                      frontend: *mut c_void,
                      backend: *mut c_void)
     -> c_int;
    pub fn zmq_sendmsg(s: *mut c_void, msg: *mut zmq_msg_t,
                       flags: c_int) -> c_int;
    pub fn zmq_recvmsg(s: *mut c_void, msg: *mut zmq_msg_t,
                       flags: c_int) -> c_int;
    pub fn zmq_z85_encode(dest: *mut c_char,
                          data: *const uint8_t, size: size_t)
     -> *mut c_char;
    pub fn zmq_z85_decode(dest: *mut uint8_t,
                          string: *const c_char)
     -> *mut uint8_t;
    pub fn zmq_curve_keypair(z85_public_key: *mut c_char,
                             z85_secret_key: *mut c_char)
     -> c_int;
    pub fn zmq_sendiov(s: *mut c_void, iov: *mut Struct_iovec,
                       count: size_t, flags: c_int)
     -> c_int;
    pub fn zmq_recviov(s: *mut c_void, iov: *mut Struct_iovec,
                       count: *mut size_t, flags: c_int)
     -> c_int;
    pub fn zmq_stopwatch_start() -> *mut c_void;
    pub fn zmq_stopwatch_stop(watch_: *mut c_void)
     -> c_ulong;
    pub fn zmq_sleep(seconds_: c_int);
    pub fn zmq_threadstart(func: zmq_thread_fn,
                           arg: *mut c_void)
     -> *mut c_void;
    pub fn zmq_threadclose(thread: *mut c_void);
}
