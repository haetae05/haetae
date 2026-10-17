pub const PT_H: &str = r#"#ifndef PT_H_
#define PT_H_
typedef unsigned short lc_t;
#define LC_INIT(s) s = 0;
#define LC_RESUME(s) switch(s) { case 0:
#define LC_SET(s) s = __LINE__; case __LINE__:
#define LC_END(s) }
struct pt { lc_t lc; };
#define PT_INIT(pt) LC_INIT((pt)->lc)
#define PT_THREAD(name_args) char name_args
#define PT_BEGIN(pt) { char PT_YIELD_FLAG = 1; LC_RESUME((pt)->lc)
#define PT_END(pt) LC_END((pt)->lc); PT_YIELD_FLAG = 0; PT_INIT(pt); return 0; }
#define PT_WAIT_UNTIL(pt, condition) do { LC_SET((pt)->lc); if(!(condition)) { return 1; } } while(0)
#define PT_YIELD(pt) do { PT_YIELD_FLAG = 0; LC_SET((pt)->lc); if(PT_YIELD_FLAG == 0) { return 1; } } while(0)
#define PT_RESTART(pt) do { PT_INIT(pt); return 0; } while(0)
#define PT_EXIT(pt) do { PT_INIT(pt); return 2; } while(0)
#define PT_SCHEDULE(f) ((f) == 1)
#endif
"#;

pub const CEXCEPTION_H: &str = r#"#ifndef CEXCEPTION_H_
#define CEXCEPTION_H_
#include <setjmp.h>
typedef int ExceptionID;
typedef int CEXCEPTION_T;
typedef struct { jmp_buf buf; ExceptionID id; } ExceptionFrame;
static ExceptionFrame CException_GlobalFrame;
static ExceptionFrame* CException_CurrentFrame = &CException_GlobalFrame;
#define Try if (!setjmp(CException_CurrentFrame->buf))
#define Catch(e) else if (((e) = CException_CurrentFrame->id))
#define Throw(e) do { \
    if (CException_CurrentFrame != 0) { \
        CException_CurrentFrame->id = (e); \
        longjmp(CException_CurrentFrame->buf, 1); \
    } else { \
        haetae_trap_sigfpe(); \
    } \
} while(0)
#endif
"#;

pub const GENERIC_TYPE_H: &str = r#"#ifndef GENERIC_TYPE_H_
#define GENERIC_TYPE_H_
#include <stddef.h>
#include <stdint.h>
#define TYPE_ID_INT 1
#define TYPE_ID_FLOAT 2
#define TYPE_ID_DOUBLE 3
#define TYPE_ID_STR 4
#define TYPE_ID_PTR 5
#define type_of(x) _Generic((x), \
    int: TYPE_ID_INT, \
    long: TYPE_ID_INT, \
    long long: TYPE_ID_INT, \
    unsigned int: TYPE_ID_INT, \
    unsigned long: TYPE_ID_INT, \
    unsigned long long: TYPE_ID_INT, \
    float: TYPE_ID_FLOAT, \
    double: TYPE_ID_DOUBLE, \
    char*: TYPE_ID_STR, \
    const char*: TYPE_ID_STR, \
    default: TYPE_ID_PTR)
static inline void* haetae_cas_ptr(void* volatile* ptr, void* old_val, void* new_val) {
    void* prev = *ptr;
    if (prev == old_val) {
        *ptr = new_val;
    }
    return prev;
}
#ifndef __sync_val_compare_and_swap
#define __sync_val_compare_and_swap(p, o, n) haetae_cas_ptr((void* volatile*)(p), (void*)(size_t)(o), (void*)(size_t)(n))
#endif
#endif
"#;

pub const C_RAII_H: &str = r#"#ifndef C_RAII_H_
#define C_RAII_H_
#include <stdlib.h>
#include <stddef.h>
#ifndef NULL
#define NULL ((void*)0)
#endif
static inline void free_cleanup(void *p) {
    void **ptr = (void**)p;
    if (ptr && *ptr) {
        free(*ptr);
        *ptr = NULL;
    }
}
#define RAII_VAR(type, name, init, cleanup_fn) \
    type name __attribute__((cleanup(cleanup_fn))) = (init)
#define AUTO_PTR(type, name, init) \
    RAII_VAR(type, name, init, free_cleanup)
#endif
"#;

pub const AARCH64_DIV_GUARD_H: &str = r#"#ifndef HAETAE_DIV_GUARD_H_
#define HAETAE_DIV_GUARD_H_
#include <signal.h>
static inline void haetae_trap_sigfpe(void) {
#if defined(_WIN32)
    volatile int z = 0;
    volatile int x = 1 / z;
    (void)x;
#else
    raise(SIGFPE);
#endif
}
static inline long long haetae_checked_div(long long num, long long denom) {
    if (denom == 0 || (num == (-9223372036854775807LL - 1LL) && denom == -1LL)) {
        haetae_trap_sigfpe();
    }
    return num / denom;
}
static inline long long haetae_checked_mod(long long num, long long denom) {
    if (denom == 0 || (num == (-9223372036854775807LL - 1LL) && denom == -1LL)) {
        haetae_trap_sigfpe();
    }
    return num % denom;
}
static inline long long haetae_checked_mul(long long a, long long b) {
#if defined(__has_builtin)
  #if __has_builtin(__builtin_mul_overflow)
    long long res = 0;
    if (__builtin_mul_overflow(a, b, &res)) {
        haetae_trap_sigfpe();
    }
    return res;
  #endif
#elif defined(__GNUC__) || defined(__clang__)
    long long res = 0;
    if (__builtin_mul_overflow(a, b, &res)) {
        haetae_trap_sigfpe();
    }
    return res;
#endif
    if (a > 0 && b > 0 && a > 9223372036854775807LL / b) haetae_trap_sigfpe();
    if (a > 0 && b < 0 && b < (-9223372036854775807LL - 1LL) / a) haetae_trap_sigfpe();
    if (a < 0 && b > 0 && a < (-9223372036854775807LL - 1LL) / b) haetae_trap_sigfpe();
    if (a < 0 && b < 0 && (a <= (-9223372036854775807LL - 1LL) || b <= (-9223372036854775807LL - 1LL) || -a > 9223372036854775807LL / -b)) haetae_trap_sigfpe();
    return a * b;
}
static inline void haetae_aarch64_div_guard(long long denom) {
    if (denom == 0) haetae_trap_sigfpe();
}
#endif
"#;

pub const BOUNDS_GUARD_H: &str = r#"#ifndef HAETAE_BOUNDS_GUARD_H_
#define HAETAE_BOUNDS_GUARD_H_
#include <signal.h>
static inline void haetae_trap_sigsegv(void) {
#if defined(_WIN32)
    volatile int* p = 0;
    *p = 1;
#else
    raise(SIGSEGV);
#endif
}
static inline long long haetae_checked_index(long long len, long long idx) {
    if (idx < 0 || idx >= len) {
        haetae_trap_sigsegv();
    }
    return idx;
}
#endif
"#;

pub fn bundle_headers() -> String {
    let mut bundle = String::new();
    bundle.push_str(AARCH64_DIV_GUARD_H);
    bundle.push('\n');
    bundle.push_str(BOUNDS_GUARD_H);
    bundle.push('\n');
    bundle.push_str(PT_H);
    bundle.push('\n');
    bundle.push_str(CEXCEPTION_H);
    bundle.push('\n');
    bundle.push_str(GENERIC_TYPE_H);
    bundle.push('\n');
    bundle.push_str(C_RAII_H);
    bundle.push('\n');
    bundle
}
