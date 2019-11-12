// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use core::result;
use core::fmt;
use crate::int32_t;

//
// sgx_error.h
//
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
    pub enum sgx_status_t {
        SGX_SUCCESS                         = 0x0000_0000,

        SGX_ERROR_UNEXPECTED                = 0x0000_0001,      /* Unexpected error */
        SGX_ERROR_INVALID_PARAMETER         = 0x0000_0002,      /* The parameter is incorrect */
        SGX_ERROR_OUT_OF_MEMORY             = 0x0000_0003,      /* Not enough memory is available to complete this operation */
        SGX_ERROR_ENCLAVE_LOST              = 0x0000_0004,      /* Enclave lost after power transition or used in child process created by linux:fork() */
        SGX_ERROR_INVALID_STATE             = 0x0000_0005,      /* SGX API is invoked in incorrect order or state */
        SGX_ERROR_FEATURE_NOT_SUPPORTED     = 0x0000_0008,      /* Feature is not supported on this platform */

        SGX_ERROR_INVALID_FUNCTION   = 0x0000_1001,      /* The ecall/ocall index is invalid */
        SGX_ERROR_OUT_OF_TCS         = 0x0000_1003,      /* The enclave is out of TCS */
        SGX_ERROR_ENCLAVE_CRASHED    = 0x0000_1006,      /* The enclave is crashed */
        SGX_ERROR_ECALL_NOT_ALLOWED  = 0x0000_1007,      /* The ECALL is not allowed at this time, e.g. ecall is blocked by the dynamic entry table, or nested ecall is not allowed during initialization */
        SGX_ERROR_OCALL_NOT_ALLOWED  = 0x0000_1008,      /* The OCALL is not allowed at this time, e.g. ocall is not allowed during exception handling */
        SGX_ERROR_STACK_OVERRUN      = 0x0000_1009,      /* The enclave is running out of stack */

        SGX_ERROR_UNDEFINED_SYMBOL      = 0x0000_2000,      /* The enclave image has undefined symbol. */
        SGX_ERROR_INVALID_ENCLAVE       = 0x0000_2001,      /* The enclave image is not correct. */
        SGX_ERROR_INVALID_ENCLAVE_ID    = 0x0000_2002,      /* The enclave id is invalid */
        SGX_ERROR_INVALID_SIGNATURE     = 0x0000_2003,      /* The signature is invalid */
        SGX_ERROR_NDEBUG_ENCLAVE        = 0x0000_2004,      /* The enclave is signed as product enclave, and can not be created as debuggable enclave. */
        SGX_ERROR_OUT_OF_EPC            = 0x0000_2005,      /* Not enough EPC is available to load the enclave */
        SGX_ERROR_NO_DEVICE             = 0x0000_2006,      /* Can't open SGX device */
        SGX_ERROR_MEMORY_MAP_CONFLICT   = 0x0000_2007,      /* Page mapping failed in driver */
        SGX_ERROR_INVALID_METADATA      = 0x0000_2009,      /* The metadata is incorrect. */
        SGX_ERROR_DEVICE_BUSY           = 0x0000_200c,      /* Device is busy, mostly EINIT failed. */
        SGX_ERROR_INVALID_VERSION       = 0x0000_200d,      /* Metadata version is inconsistent between uRTS and sgx_sign or uRTS is incompatible with current platform. */
        SGX_ERROR_MODE_INCOMPATIBLE     = 0x0000_200e,      /* The target enclave 32/64 bit mode or sim/hw mode is incompatible with the mode of current uRTS. */
        SGX_ERROR_ENCLAVE_FILE_ACCESS   = 0x0000_200f,      /* Can't open enclave file. */
        SGX_ERROR_INVALID_MISC          = 0x0000_2010,      /* The MiscSelct/MiscMask settings are not correct.*/
        SGX_ERROR_INVALID_LAUNCH_TOKEN  = 0x0000_2011,      /* The launch token is not correct.*/

        SGX_ERROR_MAC_MISMATCH       = 0x0000_3001,      /* Indicates verification error for reports, sealed datas, etc */
        SGX_ERROR_INVALID_ATTRIBUTE  = 0x0000_3002,      /* The enclave is not authorized */
        SGX_ERROR_INVALID_CPUSVN     = 0x0000_3003,      /* The cpu svn is beyond platform's cpu svn value */
        SGX_ERROR_INVALID_ISVSVN     = 0x0000_3004,      /* The isv svn is greater than the enclave's isv svn */
        SGX_ERROR_INVALID_KEYNAME    = 0x0000_3005,      /* The key name is an unsupported value */

        SGX_ERROR_SERVICE_UNAVAILABLE       = 0x0000_4001,   /* Indicates aesm didn't respond or the requested service is not supported */
        SGX_ERROR_SERVICE_TIMEOUT           = 0x0000_4002,   /* The request to aesm timed out */
        SGX_ERROR_AE_INVALID_EPIDBLOB       = 0x0000_4003,   /* Indicates epid blob verification error */
        SGX_ERROR_SERVICE_INVALID_PRIVILEGE = 0x0000_4004,   /* Enclave has no privilege to get launch token */
        SGX_ERROR_EPID_MEMBER_REVOKED       = 0x0000_4005,   /* The EPID group membership is revoked. */
        SGX_ERROR_UPDATE_NEEDED             = 0x0000_4006,   /* SGX needs to be updated */
        SGX_ERROR_NETWORK_FAILURE           = 0x0000_4007,   /* Network connecting or proxy setting issue is encountered */
        SGX_ERROR_AE_SESSION_INVALID        = 0x0000_4008,   /* Session is invalid or ended by server */
        SGX_ERROR_BUSY                      = 0x0000_400a,   /* The requested service is temporarily not availabe */
        SGX_ERROR_MC_NOT_FOUND              = 0x0000_400c,   /* The Monotonic Counter doesn't exist or has been invalided */
        SGX_ERROR_MC_NO_ACCESS_RIGHT        = 0x0000_400d,   /* Caller doesn't have the access right to specified VMC */
        SGX_ERROR_MC_USED_UP                = 0x0000_400e,   /* Monotonic counters are used out */
        SGX_ERROR_MC_OVER_QUOTA             = 0x0000_400f,   /* Monotonic counters exceeds quota limitation */
        SGX_ERROR_KDF_MISMATCH              = 0x0000_4011,   /* Key derivation function doesn't match during key exchange */
        SGX_ERROR_UNRECOGNIZED_PLATFORM     = 0x0000_4012,   /* EPID Provisioning failed due to platform not recognized by backend server*/
        SGX_ERROR_UNSUPPORTED_CONFIG        = 0x0000_4013,   /* The config for trigging EPID Provisiong or PSE Provisiong&LTP is invalid*/

        SGX_ERROR_NO_PRIVILEGE              = 0x0000_5002,   /* Not enough privilege to perform the operation */

        /* SGX Protected Code Loader Error codes*/
        SGX_ERROR_PCL_ENCRYPTED             = 0x0000_6001,   /* trying to encrypt an already encrypted enclave */
        SGX_ERROR_PCL_NOT_ENCRYPTED         = 0x0000_6002,   /* trying to load a plain enclave using sgx_create_encrypted_enclave */
        SGX_ERROR_PCL_MAC_MISMATCH          = 0x0000_6003,   /* section mac result does not match build time mac */
        SGX_ERROR_PCL_SHA_MISMATCH          = 0x0000_6004,   /* Unsealed key MAC does not match MAC of key hardcoded in enclave binary */
        SGX_ERROR_PCL_GUID_MISMATCH         = 0x0000_6005,   /* GUID in sealed blob does not match GUID hardcoded in enclave binary */

        /* SGX errors are only used in the file API when there is no appropriate EXXX (EINVAL, EIO etc.) error code */
        SGX_ERROR_FILE_BAD_STATUS               = 0x0000_7001,	/* The file is in bad status, run sgx_clearerr to try and fix it */
        SGX_ERROR_FILE_NO_KEY_ID                = 0x0000_7002,	/* The Key ID field is all zeros, can't re-generate the encryption key */
        SGX_ERROR_FILE_NAME_MISMATCH            = 0x0000_7003,	/* The current file name is different then the original file name (not allowed, substitution attack) */
        SGX_ERROR_FILE_NOT_SGX_FILE             = 0x0000_7004,  /* The file is not an SGX file */
        SGX_ERROR_FILE_CANT_OPEN_RECOVERY_FILE  = 0x0000_7005,	/* A recovery file can't be opened, so flush operation can't continue (only used when no EXXX is returned)  */
        SGX_ERROR_FILE_CANT_WRITE_RECOVERY_FILE = 0x0000_7006,  /* A recovery file can't be written, so flush operation can't continue (only used when no EXXX is returned)  */
        SGX_ERROR_FILE_RECOVERY_NEEDED          = 0x0000_7007,	/* When openeing the file, recovery is needed, but the recovery process failed */
        SGX_ERROR_FILE_FLUSH_FAILED             = 0x0000_7008,	/* fflush operation (to disk) failed (only used when no EXXX is returned) */
        SGX_ERROR_FILE_CLOSE_FAILED             = 0x0000_7009,	/* fclose operation (to disk) failed (only used when no EXXX is returned) */

        SGX_ERROR_UNSUPPORTED_ATT_KEY_ID        = 0x0000_8001,    /* platform quoting infrastructure does not support the key.*/
        SGX_ERROR_ATT_KEY_CERTIFICATION_FAILURE = 0x0000_8002,    /* Failed to generate and certify the attestation key.*/
        SGX_ERROR_ATT_KEY_UNINITIALIZED         = 0x0000_8003,    /* The platform quoting infrastructure does not have the attestation key available to generate quote.*/
        SGX_ERROR_INVALID_ATT_KEY_CERT_DATA     = 0x0000_8004,    /* TThe data returned by the platform library's sgx_get_quote_config() is invalid.*/
        SGX_ERROR_PLATFORM_CERT_UNAVAILABLE     = 0x0000_8005,    /* The PCK Cert for the platform is not available.*/

        SGX_INTERNAL_ERROR_ENCLAVE_CREATE_INTERRUPTED = 0x0000_F001, /* The ioctl for enclave_create unexpectedly failed with EINTR. */

        SGX_ERROR_WASM_BUFFER_TOO_SHORT         = 0x0F00_F001,   /* sgxwasm output buffer not long enough */
        SGX_ERROR_WASM_INTERPRETER_ERROR        = 0x0F00_F002,   /* sgxwasm interpreter error */
        SGX_ERROR_WASM_LOAD_MODULE_ERROR        = 0x0F00_F003,   /* sgxwasm loadmodule error */
        SGX_ERROR_WASM_TRY_LOAD_ERROR           = 0x0F00_F004,   /* sgxwasm tryload error */
        SGX_ERROR_WASM_REGISTER_ERROR           = 0x0F00_F005,   /* sgxwasm register error */
        SGX_ERROR_FAAS_BUFFER_TOO_SHORT         = 0x0F00_E001,   /* faas output buffer not long enough */
        SGX_ERROR_FAAS_INTERNAL_ERROR           = 0x0F00_E002,   /* faas exec internal error */
    }
}

impl sgx_status_t {
    pub fn __description(&self) -> &str {
        match *self {
            sgx_status_t::SGX_SUCCESS => "Success.",
            sgx_status_t::SGX_ERROR_UNEXPECTED => "Unexpected error occurred.",
            sgx_status_t::SGX_ERROR_INVALID_PARAMETER => "The parameter is incorrect.",
            sgx_status_t::SGX_ERROR_OUT_OF_MEMORY => "Not enough memory is available to complete this operation.",
            sgx_status_t::SGX_ERROR_ENCLAVE_LOST => "Enclave lost after power transition or used in child process created.",
            sgx_status_t::SGX_ERROR_INVALID_STATE => "SGX API is invoked in incorrect order or state.",
            sgx_status_t::SGX_ERROR_FEATURE_NOT_SUPPORTED => "Feature is not supported on this platform.",

            sgx_status_t::SGX_ERROR_INVALID_FUNCTION => "The ecall/ocall index is invalid.",
            sgx_status_t::SGX_ERROR_OUT_OF_TCS => "The enclave is out of TCS.",
            sgx_status_t::SGX_ERROR_ENCLAVE_CRASHED => "The enclave is crashed.",
            sgx_status_t::SGX_ERROR_ECALL_NOT_ALLOWED => "The ECALL is not allowed at this time.",
            sgx_status_t::SGX_ERROR_OCALL_NOT_ALLOWED => "The OCALL is not allowed at this time.",
            sgx_status_t::SGX_ERROR_STACK_OVERRUN => "The enclave is running out of stack.",

            sgx_status_t::SGX_ERROR_UNDEFINED_SYMBOL => "The enclave image has undefined symbol.",
            sgx_status_t::SGX_ERROR_INVALID_ENCLAVE => "The enclave image is not correct.",
            sgx_status_t::SGX_ERROR_INVALID_ENCLAVE_ID => "The enclave id is invalid.",
            sgx_status_t::SGX_ERROR_INVALID_SIGNATURE => "The signature is invalid.",
            sgx_status_t::SGX_ERROR_NDEBUG_ENCLAVE => "The enclave can not be created as debuggable enclave.",
            sgx_status_t::SGX_ERROR_OUT_OF_EPC => "Not enough EPC is available to load the enclave.",
            sgx_status_t::SGX_ERROR_NO_DEVICE => "Can't open SGX device.",
            sgx_status_t::SGX_ERROR_MEMORY_MAP_CONFLICT => "Page mapping failed in driver.",
            sgx_status_t::SGX_ERROR_INVALID_METADATA => "The metadata is incorrect.",
            sgx_status_t::SGX_ERROR_DEVICE_BUSY => "Device is busy, mostly EINIT failed.",
            sgx_status_t::SGX_ERROR_INVALID_VERSION => "Enclave version was invalid.",
            sgx_status_t::SGX_ERROR_MODE_INCOMPATIBLE => "The target enclave mode is incompatible with the mode of current uRTS.",
            sgx_status_t::SGX_ERROR_ENCLAVE_FILE_ACCESS => "Can't open enclave file.",
            sgx_status_t::SGX_ERROR_INVALID_MISC => "The MiscSelct/MiscMask settings are not correct.",
            sgx_status_t::SGX_ERROR_INVALID_LAUNCH_TOKEN => "The launch token is not correct.",

            sgx_status_t::SGX_ERROR_MAC_MISMATCH => "Indicates verification error for reports, sealed datas, etc.",
            sgx_status_t::SGX_ERROR_INVALID_ATTRIBUTE => "The enclave is not authorized.",
            sgx_status_t::SGX_ERROR_INVALID_CPUSVN => "The cpu svn is beyond platform's cpu svn value.",
            sgx_status_t::SGX_ERROR_INVALID_ISVSVN => "The isv svn is greater than the enclave's isv svn.",
            sgx_status_t::SGX_ERROR_INVALID_KEYNAME => "The key name is an unsupported value.",

            sgx_status_t::SGX_ERROR_SERVICE_UNAVAILABLE => "Indicates aesm didn't response or the requested service is not supported.",
            sgx_status_t::SGX_ERROR_SERVICE_TIMEOUT => "The request to aesm time out.",
            sgx_status_t::SGX_ERROR_AE_INVALID_EPIDBLOB => "Indicates epid blob verification error.",
            sgx_status_t::SGX_ERROR_SERVICE_INVALID_PRIVILEGE => "Enclave has no privilege to get launch token.",
            sgx_status_t::SGX_ERROR_EPID_MEMBER_REVOKED => "The EPID group membership is revoked.",
            sgx_status_t::SGX_ERROR_UPDATE_NEEDED => "SGX needs to be updated.",
            sgx_status_t::SGX_ERROR_NETWORK_FAILURE => "Network connecting or proxy setting issue is encountered.",
            sgx_status_t::SGX_ERROR_AE_SESSION_INVALID => "Session is invalid or ended by server.",
            sgx_status_t::SGX_ERROR_BUSY => "The requested service is temporarily not availabe.",
            sgx_status_t::SGX_ERROR_MC_NOT_FOUND => "The Monotonic Counter doesn't exist or has been invalided.",
            sgx_status_t::SGX_ERROR_MC_NO_ACCESS_RIGHT => "Caller doesn't have the access right to specified VMC.",
            sgx_status_t::SGX_ERROR_MC_USED_UP => "Monotonic counters are used out.",
            sgx_status_t::SGX_ERROR_MC_OVER_QUOTA => "Monotonic counters exceeds quota limitation.",
            sgx_status_t::SGX_ERROR_KDF_MISMATCH => "Key derivation function doesn't match during key exchange.",
            sgx_status_t::SGX_ERROR_UNRECOGNIZED_PLATFORM => "EPID Provisioning failed due to platform not recognized by backend server.",
            sgx_status_t::SGX_ERROR_UNSUPPORTED_CONFIG => "The config for trigging EPID Provisiong or PSE Provisiong&LTP is invalid.",
            sgx_status_t::SGX_ERROR_NO_PRIVILEGE => "Not enough privilege to perform the operation.",

            sgx_status_t::SGX_ERROR_PCL_ENCRYPTED => "Trying to encrypt an already encrypted enclave.",
            sgx_status_t::SGX_ERROR_PCL_NOT_ENCRYPTED => "Trying to load a plain enclave using sgx_create_encrypted_enclave.",
            sgx_status_t::SGX_ERROR_PCL_MAC_MISMATCH => "Section mac result does not match build time mac.",
            sgx_status_t::SGX_ERROR_PCL_SHA_MISMATCH => "Unsealed key MAC does not match MAC of key hardcoded in enclave binary.",
            sgx_status_t::SGX_ERROR_PCL_GUID_MISMATCH => "GUID in sealed blob does not match GUID hardcoded in enclave binary.",

            sgx_status_t::SGX_ERROR_FILE_BAD_STATUS => "The file is in bad status.",
            sgx_status_t::SGX_ERROR_FILE_NO_KEY_ID => "The Key ID field is all zeros, can't regenerate the encryption key.",
            sgx_status_t::SGX_ERROR_FILE_NAME_MISMATCH => "The current file name is different then the original file name.",
            sgx_status_t::SGX_ERROR_FILE_NOT_SGX_FILE => "The file is not an SGX file.",
            sgx_status_t::SGX_ERROR_FILE_CANT_OPEN_RECOVERY_FILE => "A recovery file can't be opened, so flush operation can't continue.",
            sgx_status_t::SGX_ERROR_FILE_CANT_WRITE_RECOVERY_FILE => "A recovery file can't be written, so flush operation can't continue.",
            sgx_status_t::SGX_ERROR_FILE_RECOVERY_NEEDED => "When openeing the file, recovery is needed, but the recovery process failed.",
            sgx_status_t::SGX_ERROR_FILE_FLUSH_FAILED => "fflush operation failed.",
            sgx_status_t::SGX_ERROR_FILE_CLOSE_FAILED => "fclose operation failed.",

            sgx_status_t::SGX_ERROR_UNSUPPORTED_ATT_KEY_ID => "platform quoting infrastructure does not support the key.",
            sgx_status_t::SGX_ERROR_ATT_KEY_CERTIFICATION_FAILURE => "Failed to generate and certify the attestation key.",
            sgx_status_t::SGX_ERROR_ATT_KEY_UNINITIALIZED => "The platform quoting infrastructure does not have the attestation key available to generate quote.",
            sgx_status_t::SGX_ERROR_INVALID_ATT_KEY_CERT_DATA => "The data returned by the platform library is invalid.",
            sgx_status_t::SGX_ERROR_PLATFORM_CERT_UNAVAILABLE => "The PCK Cert for the platform is not available.",

            sgx_status_t::SGX_INTERNAL_ERROR_ENCLAVE_CREATE_INTERRUPTED => "The ioctl for enclave_create unexpectedly failed with EINTR.",

            sgx_status_t::SGX_ERROR_WASM_BUFFER_TOO_SHORT => "sgx wasm output buffer too small.",
            sgx_status_t::SGX_ERROR_WASM_INTERPRETER_ERROR => "sgx wasm interpreter error.",
            sgx_status_t::SGX_ERROR_WASM_LOAD_MODULE_ERROR => "sgxwasm loadmodule error.",
            sgx_status_t::SGX_ERROR_WASM_TRY_LOAD_ERROR => "sgxwasm tryload error.",
            sgx_status_t::SGX_ERROR_WASM_REGISTER_ERROR => "sgxwasm register error.",
            sgx_status_t::SGX_ERROR_FAAS_BUFFER_TOO_SHORT => "faas output buffer too short.",
            sgx_status_t::SGX_ERROR_FAAS_INTERNAL_ERROR => "faas exec internal error.",
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            sgx_status_t::SGX_SUCCESS => "SGX_SUCCESS.",
            sgx_status_t::SGX_ERROR_UNEXPECTED => "SGX_ERROR_UNEXPECTED",
            sgx_status_t::SGX_ERROR_INVALID_PARAMETER => "SGX_ERROR_INVALID_PARAMETER",
            sgx_status_t::SGX_ERROR_OUT_OF_MEMORY => "SGX_ERROR_OUT_OF_MEMORY",
            sgx_status_t::SGX_ERROR_ENCLAVE_LOST => "SGX_ERROR_ENCLAVE_LOST",
            sgx_status_t::SGX_ERROR_INVALID_STATE => "SGX_ERROR_INVALID_STATE",
            sgx_status_t::SGX_ERROR_FEATURE_NOT_SUPPORTED => "SGX_ERROR_FEATURE_NOT_SUPPORTED",

            sgx_status_t::SGX_ERROR_INVALID_FUNCTION => "SGX_ERROR_INVALID_FUNCTION",
            sgx_status_t::SGX_ERROR_OUT_OF_TCS => "SGX_ERROR_OUT_OF_TCS",
            sgx_status_t::SGX_ERROR_ENCLAVE_CRASHED => "SGX_ERROR_ENCLAVE_CRASHED",
            sgx_status_t::SGX_ERROR_ECALL_NOT_ALLOWED => "SGX_ERROR_ECALL_NOT_ALLOWED",
            sgx_status_t::SGX_ERROR_OCALL_NOT_ALLOWED => "SGX_ERROR_OCALL_NOT_ALLOWED",
            sgx_status_t::SGX_ERROR_STACK_OVERRUN => "SGX_ERROR_STACK_OVERRUN",

            sgx_status_t::SGX_ERROR_UNDEFINED_SYMBOL => "SGX_ERROR_UNDEFINED_SYMBOL",
            sgx_status_t::SGX_ERROR_INVALID_ENCLAVE => "SGX_ERROR_INVALID_ENCLAVE",
            sgx_status_t::SGX_ERROR_INVALID_ENCLAVE_ID => "SGX_ERROR_INVALID_ENCLAVE_ID",
            sgx_status_t::SGX_ERROR_INVALID_SIGNATURE => "SGX_ERROR_INVALID_SIGNATURE",
            sgx_status_t::SGX_ERROR_NDEBUG_ENCLAVE => "SGX_ERROR_NDEBUG_ENCLAVE",
            sgx_status_t::SGX_ERROR_OUT_OF_EPC => "SGX_ERROR_OUT_OF_EPC",
            sgx_status_t::SGX_ERROR_NO_DEVICE => "SGX_ERROR_NO_DEVICE",
            sgx_status_t::SGX_ERROR_MEMORY_MAP_CONFLICT => "SGX_ERROR_MEMORY_MAP_CONFLICT",
            sgx_status_t::SGX_ERROR_INVALID_METADATA => "SGX_ERROR_INVALID_METADATA",
            sgx_status_t::SGX_ERROR_DEVICE_BUSY => "SGX_ERROR_DEVICE_BUSY",
            sgx_status_t::SGX_ERROR_INVALID_VERSION => "SGX_ERROR_INVALID_VERSION",
            sgx_status_t::SGX_ERROR_MODE_INCOMPATIBLE => "SGX_ERROR_MODE_INCOMPATIBLE",
            sgx_status_t::SGX_ERROR_ENCLAVE_FILE_ACCESS => "SGX_ERROR_ENCLAVE_FILE_ACCESS",
            sgx_status_t::SGX_ERROR_INVALID_MISC => "SGX_ERROR_INVALID_MISC",
            sgx_status_t::SGX_ERROR_INVALID_LAUNCH_TOKEN => "SGX_ERROR_INVALID_LAUNCH_TOKEN",

            sgx_status_t::SGX_ERROR_MAC_MISMATCH => "SGX_ERROR_MAC_MISMATCH",
            sgx_status_t::SGX_ERROR_INVALID_ATTRIBUTE => "SGX_ERROR_INVALID_ATTRIBUTE",
            sgx_status_t::SGX_ERROR_INVALID_CPUSVN => "SGX_ERROR_INVALID_CPUSVN",
            sgx_status_t::SGX_ERROR_INVALID_ISVSVN => "SGX_ERROR_INVALID_ISVSVN",
            sgx_status_t::SGX_ERROR_INVALID_KEYNAME => "SGX_ERROR_INVALID_KEYNAME",

            sgx_status_t::SGX_ERROR_SERVICE_UNAVAILABLE => "SGX_ERROR_SERVICE_UNAVAILABLE",
            sgx_status_t::SGX_ERROR_SERVICE_TIMEOUT => "SGX_ERROR_SERVICE_TIMEOUT",
            sgx_status_t::SGX_ERROR_AE_INVALID_EPIDBLOB => "SGX_ERROR_AE_INVALID_EPIDBLOB",
            sgx_status_t::SGX_ERROR_SERVICE_INVALID_PRIVILEGE => "SGX_ERROR_SERVICE_INVALID_PRIVILEGE",
            sgx_status_t::SGX_ERROR_EPID_MEMBER_REVOKED => "SGX_ERROR_EPID_MEMBER_REVOKED",
            sgx_status_t::SGX_ERROR_UPDATE_NEEDED => "SGX_ERROR_UPDATE_NEEDED",
            sgx_status_t::SGX_ERROR_NETWORK_FAILURE => "SGX_ERROR_NETWORK_FAILURE",
            sgx_status_t::SGX_ERROR_AE_SESSION_INVALID => "SGX_ERROR_AE_SESSION_INVALID",
            sgx_status_t::SGX_ERROR_BUSY => "SGX_ERROR_BUSY",
            sgx_status_t::SGX_ERROR_MC_NOT_FOUND => "SGX_ERROR_MC_NOT_FOUND",
            sgx_status_t::SGX_ERROR_MC_NO_ACCESS_RIGHT => "SGX_ERROR_MC_NO_ACCESS_RIGHT",
            sgx_status_t::SGX_ERROR_MC_USED_UP => "SGX_ERROR_MC_USED_UP",
            sgx_status_t::SGX_ERROR_MC_OVER_QUOTA => "SGX_ERROR_MC_OVER_QUOTA",
            sgx_status_t::SGX_ERROR_KDF_MISMATCH => "SGX_ERROR_KDF_MISMATCH",
            sgx_status_t::SGX_ERROR_UNRECOGNIZED_PLATFORM => "SGX_ERROR_UNRECOGNIZED_PLATFORM",
            sgx_status_t::SGX_ERROR_UNSUPPORTED_CONFIG => "SGX_ERROR_UNSUPPORTED_CONFIG",
            sgx_status_t::SGX_ERROR_NO_PRIVILEGE => "SGX_ERROR_NO_PRIVILEGE",

            sgx_status_t::SGX_ERROR_PCL_ENCRYPTED => "SGX_ERROR_PCL_ENCRYPTED",
            sgx_status_t::SGX_ERROR_PCL_NOT_ENCRYPTED => "SGX_ERROR_PCL_NOT_ENCRYPTED",
            sgx_status_t::SGX_ERROR_PCL_MAC_MISMATCH => "SGX_ERROR_PCL_MAC_MISMATCH",
            sgx_status_t::SGX_ERROR_PCL_SHA_MISMATCH => "SGX_ERROR_PCL_SHA_MISMATCH",
            sgx_status_t::SGX_ERROR_PCL_GUID_MISMATCH => "SGX_ERROR_PCL_GUID_MISMATCH",

            sgx_status_t::SGX_ERROR_FILE_BAD_STATUS => "SGX_ERROR_FILE_BAD_STATUS",
            sgx_status_t::SGX_ERROR_FILE_NO_KEY_ID => "SGX_ERROR_FILE_NO_KEY_ID",
            sgx_status_t::SGX_ERROR_FILE_NAME_MISMATCH => "SGX_ERROR_FILE_NAME_MISMATCH",
            sgx_status_t::SGX_ERROR_FILE_NOT_SGX_FILE => "SGX_ERROR_FILE_NOT_SGX_FILE",
            sgx_status_t::SGX_ERROR_FILE_CANT_OPEN_RECOVERY_FILE => "SGX_ERROR_FILE_CANT_OPEN_RECOVERY_FILE",
            sgx_status_t::SGX_ERROR_FILE_CANT_WRITE_RECOVERY_FILE => "SGX_ERROR_FILE_CANT_WRITE_RECOVERY_FILE",
            sgx_status_t::SGX_ERROR_FILE_RECOVERY_NEEDED => "SGX_ERROR_FILE_RECOVERY_NEEDED",
            sgx_status_t::SGX_ERROR_FILE_FLUSH_FAILED => "SGX_ERROR_FILE_FLUSH_FAILED",
            sgx_status_t::SGX_ERROR_FILE_CLOSE_FAILED => "SGX_ERROR_FILE_CLOSE_FAILED",

            sgx_status_t::SGX_ERROR_UNSUPPORTED_ATT_KEY_ID => "SGX_ERROR_UNSUPPORTED_ATT_KEY_ID",
            sgx_status_t::SGX_ERROR_ATT_KEY_CERTIFICATION_FAILURE => "SGX_ERROR_ATT_KEY_CERTIFICATION_FAILURE",
            sgx_status_t::SGX_ERROR_ATT_KEY_UNINITIALIZED => "SGX_ERROR_ATT_KEY_UNINITIALIZED",
            sgx_status_t::SGX_ERROR_INVALID_ATT_KEY_CERT_DATA => "SGX_ERROR_INVALID_ATT_KEY_CERT_DATA",
            sgx_status_t::SGX_ERROR_PLATFORM_CERT_UNAVAILABLE => "SGX_ERROR_PLATFORM_CERT_UNAVAILABLE",

            sgx_status_t::SGX_INTERNAL_ERROR_ENCLAVE_CREATE_INTERRUPTED => "SGX_INTERNAL_ERROR_ENCLAVE_CREATE_INTERRUPTED",

            sgx_status_t::SGX_ERROR_WASM_BUFFER_TOO_SHORT => "SGX_ERROR_WASM_BUFFER_TOO_SHORT",
            sgx_status_t::SGX_ERROR_WASM_INTERPRETER_ERROR => "SGX_ERROR_WASM_INTERPRETER_ERROR",
            sgx_status_t::SGX_ERROR_WASM_LOAD_MODULE_ERROR => "SGX_ERROR_WASM_LOAD_MODULE_ERROR",
            sgx_status_t::SGX_ERROR_WASM_TRY_LOAD_ERROR    => "SGX_ERROR_WASM_TRY_LOAD_ERROR",
            sgx_status_t::SGX_ERROR_WASM_REGISTER_ERROR    => "SGX_ERROR_WASM_REGISTER_ERROR",
            sgx_status_t::SGX_ERROR_FAAS_BUFFER_TOO_SHORT   => "SGX_ERROR_FAAS_BUFFER_TOO_SHORT",
            sgx_status_t::SGX_ERROR_FAAS_INTERNAL_ERROR => "SGX_ERROR_FAAS_INTERNAL_ERROR",
        }
    }
}

impl fmt::Display for sgx_status_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
    pub enum sgx_pce_error_t {
        SGX_PCE_SUCCESS                 = 0x0000_F000,
        SGX_PCE_UNEXPECTED              = 0x0000_F001,
        SGX_PCE_INVALID_PARAMETER       = 0x0000_F002,
        SGX_PCE_OUT_OF_EPC              = 0x0000_F003,
        SGX_PCE_INTERFACE_UNAVAILABLE   = 0x0000_F004,
        SGX_PCE_INVALID_REPORT          = 0x0000_F005,
        SGX_PCE_CRYPTO_ERROR            = 0x0000_F006,
        SGX_PCE_INVALID_PRIVILEGE       = 0x0000_F007,
        SGX_PCE_INVALID_TCB             = 0x0000_F008,
    }
}

impl sgx_pce_error_t {
    pub fn __description(&self) -> &str {
        match *self {
            sgx_pce_error_t::SGX_PCE_SUCCESS => "Success.",
            sgx_pce_error_t::SGX_PCE_UNEXPECTED => "Unexpected error.",
            sgx_pce_error_t::SGX_PCE_INVALID_PARAMETER => "The parameter is incorrect.",
            sgx_pce_error_t::SGX_PCE_OUT_OF_EPC => "Not enough memory is available to complete this operation.",
            sgx_pce_error_t::SGX_PCE_INTERFACE_UNAVAILABLE => "SGX API is unavailable.",
            sgx_pce_error_t::SGX_PCE_INVALID_REPORT => "The report cannot be verified.",
            sgx_pce_error_t::SGX_PCE_CRYPTO_ERROR => "Cannot decrypt or verify ciphertext.",
            sgx_pce_error_t::SGX_PCE_INVALID_PRIVILEGE => "Not enough privilege to perform the operation.",
            sgx_pce_error_t::SGX_PCE_INVALID_TCB => "PCE could not sign at the requested TCB.",
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            sgx_pce_error_t::SGX_PCE_SUCCESS => "SGX_PCE_SUCCESS.",
            sgx_pce_error_t::SGX_PCE_UNEXPECTED => "SGX_PCE_UNEXPECTED",
            sgx_pce_error_t::SGX_PCE_INVALID_PARAMETER => "SGX_PCE_INVALID_PARAMETER",
            sgx_pce_error_t::SGX_PCE_OUT_OF_EPC => "SGX_PCE_OUT_OF_EPC",
            sgx_pce_error_t::SGX_PCE_INTERFACE_UNAVAILABLE => "SGX_PCE_INTERFACE_UNAVAILABLE",
            sgx_pce_error_t::SGX_PCE_INVALID_REPORT => "SGX_PCE_INVALID_REPORT",
            sgx_pce_error_t::SGX_PCE_CRYPTO_ERROR => "SGX_PCE_CRYPTO_ERROR",
            sgx_pce_error_t::SGX_PCE_INVALID_PRIVILEGE => "SGX_PCE_INVALID_PRIVILEGE",
            sgx_pce_error_t::SGX_PCE_INVALID_TCB => "SGX_PCE_INVALID_TCB",
        }
    }
}

impl fmt::Display for sgx_pce_error_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
    pub enum sgx_quote3_error_t {
        SGX_QL_SUCCESS                                  = 0x0000_0000,
//      SGX_QL_ERROR_MIN                                = 0x0000_E001,
        SGX_QL_ERROR_UNEXPECTED                         = 0x0000_E001,
        SGX_QL_ERROR_INVALID_PARAMETER                  = 0x0000_E002,
        SGX_QL_ERROR_OUT_OF_MEMORY                      = 0x0000_E003,
        SGX_QL_ERROR_ECDSA_ID_MISMATCH                  = 0x0000_E004,
        SGX_QL_PATHNAME_BUFFER_OVERFLOW_ERROR           = 0x0000_E005,
        SGX_QL_FILE_ACCESS_ERROR                        = 0x0000_E006,
        SGX_QL_ERROR_STORED_KEY                         = 0x0000_E007,
        SGX_QL_ERROR_PUB_KEY_ID_MISMATCH                = 0x0000_E008,
        SGX_QL_ERROR_INVALID_PCE_SIG_SCHEME             = 0x0000_E009,
        SGX_QL_ATT_KEY_BLOB_ERROR                       = 0x0000_E00A,
        SGX_QL_UNSUPPORTED_ATT_KEY_ID                   = 0x0000_E00B,
        SGX_QL_UNSUPPORTED_LOADING_POLICY               = 0x0000_E00C,
        SGX_QL_INTERFACE_UNAVAILABLE                    = 0x0000_E00D,
        SGX_QL_PLATFORM_LIB_UNAVAILABLE                 = 0x0000_E00E,
        SGX_QL_ATT_KEY_NOT_INITIALIZED                  = 0x0000_E00F,
        SGX_QL_ATT_KEY_CERT_DATA_INVALID                = 0x0000_E010,
        SGX_QL_NO_PLATFORM_CERT_DATA                    = 0x0000_E011,
        SGX_QL_OUT_OF_EPC                               = 0x0000_E012,
        SGX_QL_ERROR_REPORT                             = 0x0000_E013,
        SGX_QL_ENCLAVE_LOST                             = 0x0000_E014,
        SGX_QL_INVALID_REPORT                           = 0x0000_E015,
        SGX_QL_ENCLAVE_LOAD_ERROR                       = 0x0000_E016,
        SGX_QL_UNABLE_TO_GENERATE_QE_REPORT             = 0x0000_E017,
        SGX_QL_KEY_CERTIFCATION_ERROR                   = 0x0000_E018,
        SGX_QL_NETWORK_ERROR                            = 0x0000_E019,
        SGX_QL_MESSAGE_ERROR                            = 0x0000_E01A,
//      SGX_QL_ERROR_INVALID_PRIVILEGE                  = 0x0000_E01B,   dcap 1.3 define 0xE035
        SGX_QL_NO_QUOTE_COLLATERAL_DATA                 = 0x0000_E01B,
        SGX_QL_QUOTE_CERTIFICATION_DATA_UNSUPPORTED     = 0x0000_E01C,
        SGX_QL_QUOTE_FORMAT_UNSUPPORTED                 = 0x0000_E01D,
        SGX_QL_UNABLE_TO_GENERATE_REPORT                = 0x0000_E01E,
        SGX_QL_QE_REPORT_INVALID_SIGNATURE              = 0x0000_E01F,
        SGX_QL_QE_REPORT_UNSUPPORTED_FORMAT             = 0x0000_E020,
        SGX_QL_PCK_CERT_UNSUPPORTED_FORMAT              = 0x0000_E021,
        SGX_QL_PCK_CERT_CHAIN_ERROR                     = 0x0000_E022,
        SGX_QL_TCBINFO_UNSUPPORTED_FORMAT               = 0x0000_E023,
        SGX_QL_TCBINFO_MISMATCH                         = 0x0000_E024,
        SGX_QL_QEIDENTITY_UNSUPPORTED_FORMAT            = 0x0000_E025,
        SGX_QL_QEIDENTITY_MISMATCH                      = 0x0000_E026,
        SGX_QL_TCB_OUT_OF_DATE                          = 0x0000_E027,
        SGX_QL_TCB_OUT_OF_DATE_CONFIGURATION_NEEDED     = 0x0000_E028,
        SGX_QL_SGX_ENCLAVE_IDENTITY_OUT_OF_DATE         = 0x0000_E029,
        SGX_QL_SGX_ENCLAVE_REPORT_ISVSVN_OUT_OF_DATE    = 0x0000_E02A,
        SGX_QL_QE_IDENTITY_OUT_OF_DATE                  = 0x0000_E02B,
        SGX_QL_SGX_TCB_INFO_EXPIRED                     = 0x0000_E02C,
        SGX_QL_SGX_PCK_CERT_CHAIN_EXPIRED               = 0x0000_E02D,
        SGX_QL_SGX_CRL_EXPIRED                          = 0x0000_E02E,
        SGX_QL_SGX_SIGNING_CERT_CHAIN_EXPIRED           = 0x0000_E02F,
        SGX_QL_SGX_ENCLAVE_IDENTITY_EXPIRED             = 0x0000_E030,
        SGX_QL_PCK_REVOKED                              = 0x0000_E031,
        SGX_QL_TCB_REVOKED                              = 0x0000_E032,
        SGX_QL_TCB_CONFIGURATION_NEEDED                 = 0x0000_E033,
        SGX_QL_UNABLE_TO_GET_COLLATERAL                 = 0x0000_E034,
        SGX_QL_ERROR_INVALID_PRIVILEGE                  = 0x0000_E035,
        SGX_QL_NO_QVE_IDENTITY_DATA                     = 0x0000_E037,
        SGX_QL_CRL_UNSUPPORTED_FORMAT                   = 0x0000_E038,
        SGX_QL_QEIDENTITY_CHAIN_ERROR                   = 0x0000_E039,
        SGX_QL_TCBINFO_CHAIN_ERROR                      = 0x0000_E03A,
        SGX_QL_ERROR_MAX                                = 0x0000_E0FF,
    }
}

impl sgx_quote3_error_t {
    pub fn __description(&self) -> &str {
        match *self {
            sgx_quote3_error_t::SGX_QL_SUCCESS => "Success.",
//          sgx_quote3_error_t::SGX_QL_ERROR_MIN => "Indicate min error to allow better translation.",
            sgx_quote3_error_t::SGX_QL_ERROR_UNEXPECTED => "Unexpected error.",
            sgx_quote3_error_t::SGX_QL_ERROR_INVALID_PARAMETER => "The parameter is incorrect",
            sgx_quote3_error_t::SGX_QL_ERROR_OUT_OF_MEMORY => "Not enough memory is available to complete this operation.",
            sgx_quote3_error_t::SGX_QL_ERROR_ECDSA_ID_MISMATCH => "Expected ECDSA_ID does not match the value stored in the ECDSA Blob.",
            sgx_quote3_error_t::SGX_QL_PATHNAME_BUFFER_OVERFLOW_ERROR => "The ECDSA blob pathname is too large.",
            sgx_quote3_error_t::SGX_QL_FILE_ACCESS_ERROR => "Error accessing ECDSA blob.",
            sgx_quote3_error_t::SGX_QL_ERROR_STORED_KEY => "Cached ECDSA key is invalid.",
            sgx_quote3_error_t::SGX_QL_ERROR_PUB_KEY_ID_MISMATCH => "Cached ECDSA key does not match requested key.",
            sgx_quote3_error_t::SGX_QL_ERROR_INVALID_PCE_SIG_SCHEME => "PCE use the incorrect signature scheme.",
            sgx_quote3_error_t::SGX_QL_ATT_KEY_BLOB_ERROR => "There is a problem with the attestation key blob.",
            sgx_quote3_error_t::SGX_QL_UNSUPPORTED_ATT_KEY_ID => "Unsupported attestation key ID.",
            sgx_quote3_error_t::SGX_QL_UNSUPPORTED_LOADING_POLICY => "Unsupported enclave loading policy.",
            sgx_quote3_error_t::SGX_QL_INTERFACE_UNAVAILABLE => "Unable to load the QE enclave.",
            sgx_quote3_error_t::SGX_QL_PLATFORM_LIB_UNAVAILABLE => "Unable to find the platform library with the dependent APIs.",
            sgx_quote3_error_t::SGX_QL_ATT_KEY_NOT_INITIALIZED => "The attestation key doesn't exist or has not been certified.",
            sgx_quote3_error_t::SGX_QL_ATT_KEY_CERT_DATA_INVALID => "The certification data retrieved from the platform library is invalid.",
            sgx_quote3_error_t::SGX_QL_NO_PLATFORM_CERT_DATA => "The platform library doesn't have any platfrom cert data.",
            sgx_quote3_error_t::SGX_QL_OUT_OF_EPC => "Not enough memory in the EPC to load the enclave.",
            sgx_quote3_error_t::SGX_QL_ERROR_REPORT => "There was a problem verifying an SGX REPORT.",
            sgx_quote3_error_t::SGX_QL_ENCLAVE_LOST => "Interfacing to the enclave failed due to a power transition.",
            sgx_quote3_error_t::SGX_QL_INVALID_REPORT => "Error verifying the application enclave's report.",
            sgx_quote3_error_t::SGX_QL_ENCLAVE_LOAD_ERROR => "Unable to load the enclaves.",
            sgx_quote3_error_t::SGX_QL_UNABLE_TO_GENERATE_QE_REPORT => "The QE was unable to generate its own report targeting the application enclave.",
            sgx_quote3_error_t::SGX_QL_KEY_CERTIFCATION_ERROR => "Caused when the provider library returns an invalid TCB.",
            sgx_quote3_error_t::SGX_QL_NETWORK_ERROR => "Network error when retrieving PCK certs.",
            sgx_quote3_error_t::SGX_QL_MESSAGE_ERROR => "Message error when retrieving PCK certs.",
            sgx_quote3_error_t::SGX_QL_NO_QUOTE_COLLATERAL_DATA => "The platform does not have the quote verification collateral data available.",
            sgx_quote3_error_t::SGX_QL_QUOTE_CERTIFICATION_DATA_UNSUPPORTED => "",
            sgx_quote3_error_t::SGX_QL_QUOTE_FORMAT_UNSUPPORTED => "",
            sgx_quote3_error_t::SGX_QL_UNABLE_TO_GENERATE_REPORT => "",
            sgx_quote3_error_t::SGX_QL_QE_REPORT_INVALID_SIGNATURE => "",
            sgx_quote3_error_t::SGX_QL_QE_REPORT_UNSUPPORTED_FORMAT => "",
            sgx_quote3_error_t::SGX_QL_PCK_CERT_UNSUPPORTED_FORMAT => "",
            sgx_quote3_error_t::SGX_QL_PCK_CERT_CHAIN_ERROR => "",
            sgx_quote3_error_t::SGX_QL_TCBINFO_UNSUPPORTED_FORMAT => "",
            sgx_quote3_error_t::SGX_QL_TCBINFO_MISMATCH => "",
            sgx_quote3_error_t::SGX_QL_QEIDENTITY_UNSUPPORTED_FORMAT => "",
            sgx_quote3_error_t::SGX_QL_QEIDENTITY_MISMATCH => "",
            sgx_quote3_error_t::SGX_QL_TCB_OUT_OF_DATE => "",
            sgx_quote3_error_t::SGX_QL_TCB_OUT_OF_DATE_CONFIGURATION_NEEDED => "",
            sgx_quote3_error_t::SGX_QL_SGX_ENCLAVE_IDENTITY_OUT_OF_DATE => "",
            sgx_quote3_error_t::SGX_QL_SGX_ENCLAVE_REPORT_ISVSVN_OUT_OF_DATE => "",
            sgx_quote3_error_t::SGX_QL_QE_IDENTITY_OUT_OF_DATE => "",
            sgx_quote3_error_t::SGX_QL_SGX_TCB_INFO_EXPIRED => "",
            sgx_quote3_error_t::SGX_QL_SGX_PCK_CERT_CHAIN_EXPIRED => "",
            sgx_quote3_error_t::SGX_QL_SGX_CRL_EXPIRED => "",
            sgx_quote3_error_t::SGX_QL_SGX_SIGNING_CERT_CHAIN_EXPIRED => "",
            sgx_quote3_error_t::SGX_QL_SGX_ENCLAVE_IDENTITY_EXPIRED => "",
            sgx_quote3_error_t::SGX_QL_PCK_REVOKED => "",
            sgx_quote3_error_t::SGX_QL_TCB_REVOKED => "",
            sgx_quote3_error_t::SGX_QL_TCB_CONFIGURATION_NEEDED => "",
            sgx_quote3_error_t::SGX_QL_UNABLE_TO_GET_COLLATERAL => "",
            sgx_quote3_error_t::SGX_QL_ERROR_INVALID_PRIVILEGE => "No enough privilege to perform the operation.",
            sgx_quote3_error_t::SGX_QL_NO_QVE_IDENTITY_DATA => "The platform does not have the QVE identity data available.",
            sgx_quote3_error_t::SGX_QL_CRL_UNSUPPORTED_FORMAT => "",
            sgx_quote3_error_t::SGX_QL_QEIDENTITY_CHAIN_ERROR => "",
            sgx_quote3_error_t::SGX_QL_TCBINFO_CHAIN_ERROR => "",
            sgx_quote3_error_t::SGX_QL_ERROR_MAX => "Indicate max error to allow better translation.",
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            sgx_quote3_error_t::SGX_QL_SUCCESS => "SGX_QL_SUCCESS",
//          sgx_quote3_error_t::SGX_QL_ERROR_MIN => "SGX_QL_ERROR_MIN",
            sgx_quote3_error_t::SGX_QL_ERROR_UNEXPECTED => "SGX_QL_ERROR_UNEXPECTED",
            sgx_quote3_error_t::SGX_QL_ERROR_INVALID_PARAMETER => "SGX_QL_ERROR_INVALID_PARAMETER",
            sgx_quote3_error_t::SGX_QL_ERROR_OUT_OF_MEMORY => "SGX_QL_ERROR_OUT_OF_MEMORY",
            sgx_quote3_error_t::SGX_QL_ERROR_ECDSA_ID_MISMATCH => "SGX_QL_ERROR_ECDSA_ID_MISMATCH",
            sgx_quote3_error_t::SGX_QL_PATHNAME_BUFFER_OVERFLOW_ERROR => "SGX_QL_PATHNAME_BUFFER_OVERFLOW_ERROR",
            sgx_quote3_error_t::SGX_QL_FILE_ACCESS_ERROR => "SGX_QL_FILE_ACCESS_ERROR",
            sgx_quote3_error_t::SGX_QL_ERROR_STORED_KEY => "SGX_QL_ERROR_STORED_KEY",
            sgx_quote3_error_t::SGX_QL_ERROR_PUB_KEY_ID_MISMATCH => "SGX_QL_ERROR_PUB_KEY_ID_MISMATCH",
            sgx_quote3_error_t::SGX_QL_ERROR_INVALID_PCE_SIG_SCHEME => "SGX_QL_ERROR_INVALID_PCE_SIG_SCHEME",
            sgx_quote3_error_t::SGX_QL_ATT_KEY_BLOB_ERROR => "SGX_QL_ATT_KEY_BLOB_ERROR",
            sgx_quote3_error_t::SGX_QL_UNSUPPORTED_ATT_KEY_ID => "SGX_QL_UNSUPPORTED_ATT_KEY_ID",
            sgx_quote3_error_t::SGX_QL_UNSUPPORTED_LOADING_POLICY => "SGX_QL_UNSUPPORTED_LOADING_POLICY",
            sgx_quote3_error_t::SGX_QL_INTERFACE_UNAVAILABLE => "SGX_QL_INTERFACE_UNAVAILABLE",
            sgx_quote3_error_t::SGX_QL_PLATFORM_LIB_UNAVAILABLE => "SGX_QL_PLATFORM_LIB_UNAVAILABLE",
            sgx_quote3_error_t::SGX_QL_ATT_KEY_NOT_INITIALIZED => "SGX_QL_ATT_KEY_NOT_INITIALIZED",
            sgx_quote3_error_t::SGX_QL_ATT_KEY_CERT_DATA_INVALID => "SGX_QL_ATT_KEY_CERT_DATA_INVALID",
            sgx_quote3_error_t::SGX_QL_NO_PLATFORM_CERT_DATA => "SGX_QL_NO_PLATFORM_CERT_DATA",
            sgx_quote3_error_t::SGX_QL_OUT_OF_EPC => "SGX_QL_OUT_OF_EPC",
            sgx_quote3_error_t::SGX_QL_ERROR_REPORT => "SGX_QL_ERROR_REPORT",
            sgx_quote3_error_t::SGX_QL_ENCLAVE_LOST => "SGX_QL_ENCLAVE_LOST",
            sgx_quote3_error_t::SGX_QL_INVALID_REPORT => "SGX_QL_INVALID_REPORT",
            sgx_quote3_error_t::SGX_QL_ENCLAVE_LOAD_ERROR => "SGX_QL_ENCLAVE_LOAD_ERROR",
            sgx_quote3_error_t::SGX_QL_UNABLE_TO_GENERATE_QE_REPORT => "SGX_QL_UNABLE_TO_GENERATE_QE_REPORT",
            sgx_quote3_error_t::SGX_QL_KEY_CERTIFCATION_ERROR => "SGX_QL_KEY_CERTIFCATION_ERROR",
            sgx_quote3_error_t::SGX_QL_NETWORK_ERROR => "SGX_QL_NETWORK_ERROR",
            sgx_quote3_error_t::SGX_QL_MESSAGE_ERROR => "SGX_QL_MESSAGE_ERROR",
            sgx_quote3_error_t::SGX_QL_NO_QUOTE_COLLATERAL_DATA => "SGX_QL_NO_QUOTE_COLLATERAL_DATA",
            sgx_quote3_error_t::SGX_QL_QUOTE_CERTIFICATION_DATA_UNSUPPORTED => "SGX_QL_QUOTE_CERTIFICATION_DATA_UNSUPPORTED",
            sgx_quote3_error_t::SGX_QL_QUOTE_FORMAT_UNSUPPORTED => "SGX_QL_QUOTE_FORMAT_UNSUPPORTED",
            sgx_quote3_error_t::SGX_QL_UNABLE_TO_GENERATE_REPORT => "SGX_QL_UNABLE_TO_GENERATE_REPORT",
            sgx_quote3_error_t::SGX_QL_QE_REPORT_INVALID_SIGNATURE => "SGX_QL_QE_REPORT_INVALID_SIGNATURE",
            sgx_quote3_error_t::SGX_QL_QE_REPORT_UNSUPPORTED_FORMAT => "SGX_QL_QE_REPORT_UNSUPPORTED_FORMAT",
            sgx_quote3_error_t::SGX_QL_PCK_CERT_UNSUPPORTED_FORMAT => "SGX_QL_PCK_CERT_UNSUPPORTED_FORMAT",
            sgx_quote3_error_t::SGX_QL_PCK_CERT_CHAIN_ERROR => "SGX_QL_PCK_CERT_CHAIN_ERROR",
            sgx_quote3_error_t::SGX_QL_TCBINFO_UNSUPPORTED_FORMAT => "SGX_QL_TCBINFO_UNSUPPORTED_FORMAT",
            sgx_quote3_error_t::SGX_QL_TCBINFO_MISMATCH => "SGX_QL_TCBINFO_MISMATCH",
            sgx_quote3_error_t::SGX_QL_QEIDENTITY_UNSUPPORTED_FORMAT => "SGX_QL_QEIDENTITY_UNSUPPORTED_FORMAT",
            sgx_quote3_error_t::SGX_QL_QEIDENTITY_MISMATCH => "SGX_QL_QEIDENTITY_MISMATCH",
            sgx_quote3_error_t::SGX_QL_TCB_OUT_OF_DATE => "SGX_QL_TCB_OUT_OF_DATE",
            sgx_quote3_error_t::SGX_QL_TCB_OUT_OF_DATE_CONFIGURATION_NEEDED => "SGX_QL_TCB_OUT_OF_DATE_CONFIGURATION_NEEDED",
            sgx_quote3_error_t::SGX_QL_SGX_ENCLAVE_IDENTITY_OUT_OF_DATE => "SGX_QL_SGX_ENCLAVE_IDENTITY_OUT_OF_DATE",
            sgx_quote3_error_t::SGX_QL_SGX_ENCLAVE_REPORT_ISVSVN_OUT_OF_DATE => "SGX_QL_SGX_ENCLAVE_REPORT_ISVSVN_OUT_OF_DATE",
            sgx_quote3_error_t::SGX_QL_QE_IDENTITY_OUT_OF_DATE => "SGX_QL_QE_IDENTITY_OUT_OF_DATE",
            sgx_quote3_error_t::SGX_QL_SGX_TCB_INFO_EXPIRED => "SGX_QL_SGX_TCB_INFO_EXPIRED",
            sgx_quote3_error_t::SGX_QL_SGX_PCK_CERT_CHAIN_EXPIRED => "SGX_QL_SGX_PCK_CERT_CHAIN_EXPIRED",
            sgx_quote3_error_t::SGX_QL_SGX_CRL_EXPIRED => "SGX_QL_SGX_CRL_EXPIRED",
            sgx_quote3_error_t::SGX_QL_SGX_SIGNING_CERT_CHAIN_EXPIRED => "SGX_QL_SGX_SIGNING_CERT_CHAIN_EXPIRED",
            sgx_quote3_error_t::SGX_QL_SGX_ENCLAVE_IDENTITY_EXPIRED => "SGX_QL_SGX_ENCLAVE_IDENTITY_EXPIRED",
            sgx_quote3_error_t::SGX_QL_PCK_REVOKED => "SGX_QL_PCK_REVOKED",
            sgx_quote3_error_t::SGX_QL_TCB_REVOKED => "SGX_QL_TCB_REVOKED",
            sgx_quote3_error_t::SGX_QL_TCB_CONFIGURATION_NEEDED => "SGX_QL_TCB_CONFIGURATION_NEEDED",
            sgx_quote3_error_t::SGX_QL_UNABLE_TO_GET_COLLATERAL => "SGX_QL_UNABLE_TO_GET_COLLATERAL",
            sgx_quote3_error_t::SGX_QL_ERROR_INVALID_PRIVILEGE => "SGX_QL_ERROR_INVALID_PRIVILEGE",
            sgx_quote3_error_t::SGX_QL_NO_QVE_IDENTITY_DATA => "SGX_QL_NO_QVE_IDENTITY_DATA",
            sgx_quote3_error_t::SGX_QL_CRL_UNSUPPORTED_FORMAT => "SGX_QL_CRL_UNSUPPORTED_FORMAT",
            sgx_quote3_error_t::SGX_QL_QEIDENTITY_CHAIN_ERROR => "SGX_QL_QEIDENTITY_CHAIN_ERROR",
            sgx_quote3_error_t::SGX_QL_TCBINFO_CHAIN_ERROR => "SGX_QL_TCBINFO_CHAIN_ERROR",
            sgx_quote3_error_t::SGX_QL_ERROR_MAX => "SGX_QL_ERROR_MAX",
        }
    }
}

impl fmt::Display for sgx_quote3_error_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
    pub enum sgx_qcnl_error_t {
        SGX_QCNL_SUCCESS                    = 0x0000_0000,
        SGX_QCNL_UNEXPECTED_ERROR           = 0x0000_F001,
        SGX_QCNL_INVALID_PARAMETER          = 0x0000_F002,
        SGX_QCNL_NETWORK_ERROR              = 0x0000_F003,
        SGX_QCNL_NETWORK_PROXY_FAIL         = 0x0000_F004,
        SGX_QCNL_NETWORK_HOST_FAIL          = 0x0000_F005,
        SGX_QCNL_NETWORK_COULDNT_CONNECT    = 0x0000_F006,
        SGX_QCNL_NETWORK_HTTP2_ERROR        = 0x0000_F007,
        SGX_QCNL_NETWORK_WRITE_ERROR        = 0x0000_F008,
        SGX_QCNL_NETWORK_OPERATION_TIMEDOUT = 0x0000_F009,
        SGX_QCNL_NETWORK_HTTPS_ERROR        = 0x0000_F00A,
        SGX_QCNL_NETWORK_UNKNOWN_OPTION     = 0x0000_F00B,
        SGX_QCNL_NETWORK_INIT_ERROR         = 0x0000_F00C,
        SGX_QCNL_MSG_ERROR                  = 0x0000_F00D,
        SGX_QCNL_ERROR_STATUS_NOT_FOUND     = 0x0000_F00E,
        SGX_QCNL_OUT_OF_MEMORY              = 0x0000_F00F,
    }
}

impl sgx_qcnl_error_t {
    pub fn __description(&self) -> &str {
        match *self {
            sgx_qcnl_error_t::SGX_QCNL_SUCCESS => "Success.",
            sgx_qcnl_error_t::SGX_QCNL_UNEXPECTED_ERROR => "Unexpected error.",
            sgx_qcnl_error_t::SGX_QCNL_INVALID_PARAMETER => "The parameter is incorrect.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_ERROR => "Network error.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_PROXY_FAIL => "Network error : Couldn't resolve proxy.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_HOST_FAIL => "Network error : Couldn't resolve host.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_COULDNT_CONNECT => "Network error : Failed to connect() to host or proxy.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_HTTP2_ERROR => "Network error : A problem was detected in the HTTP2 framing layer.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_WRITE_ERROR => "Network error : an error was returned to libcurl from a write callback.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_OPERATION_TIMEDOUT => "Network error : Operation timeout.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_HTTPS_ERROR => "Network error : A problem occurred somewhere in the SSL/TLS handshake.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_UNKNOWN_OPTION => "Network error : An option passed to libcurl is not recognized/known.",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_INIT_ERROR => "Failed to initialize CURL library.",
            sgx_qcnl_error_t::SGX_QCNL_MSG_ERROR => "HTTP message error.",
            sgx_qcnl_error_t::SGX_QCNL_ERROR_STATUS_NOT_FOUND => "Data not found.",
            sgx_qcnl_error_t::SGX_QCNL_OUT_OF_MEMORY => "Out of memory error.",
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            sgx_qcnl_error_t::SGX_QCNL_SUCCESS => "SGX_QCNL_SUCCESS.",
            sgx_qcnl_error_t::SGX_QCNL_UNEXPECTED_ERROR => "SGX_QCNL_UNEXPECTED_ERROR",
            sgx_qcnl_error_t::SGX_QCNL_INVALID_PARAMETER => "SGX_QCNL_INVALID_PARAMETER",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_ERROR => "SGX_QCNL_NETWORK_ERROR",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_PROXY_FAIL => "SGX_QCNL_NETWORK_PROXY_FAIL",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_HOST_FAIL => "SGX_QCNL_NETWORK_HOST_FAIL",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_COULDNT_CONNECT => "SGX_QCNL_NETWORK_COULDNT_CONNECT",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_HTTP2_ERROR => "SGX_QCNL_NETWORK_HTTP2_ERROR",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_WRITE_ERROR => "SGX_QCNL_NETWORK_WRITE_ERROR",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_OPERATION_TIMEDOUT => "SGX_QCNL_NETWORK_OPERATION_TIMEDOUT",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_HTTPS_ERROR => "SGX_QCNL_NETWORK_HTTPS_ERROR",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_UNKNOWN_OPTION => "SGX_QCNL_NETWORK_UNKNOWN_OPTION",
            sgx_qcnl_error_t::SGX_QCNL_NETWORK_INIT_ERROR => "SGX_QCNL_NETWORK_INIT_ERROR",
            sgx_qcnl_error_t::SGX_QCNL_MSG_ERROR => "SGX_QCNL_MSG_ERROR",
            sgx_qcnl_error_t::SGX_QCNL_ERROR_STATUS_NOT_FOUND => "SGX_QCNL_ERROR_STATUS_NOT_FOUND",
            sgx_qcnl_error_t::SGX_QCNL_OUT_OF_MEMORY => "SGX_QCNL_OUT_OF_MEMORY",
        }
    }
}

impl fmt::Display for sgx_qcnl_error_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub type sys_error_t = int32_t;

pub type SgxResult<T> = result::Result<T, sgx_status_t>;
pub type SgxError = result::Result<(), sgx_status_t>;

pub type SgxPceResult<T> = result::Result<T, sgx_pce_error_t>;
pub type SgxPceError = result::Result<(), sgx_pce_error_t>;

pub type SgxQuote3Result<T> = result::Result<T, sgx_quote3_error_t>;
pub type SgxQuote3Error = result::Result<(), sgx_quote3_error_t>;

pub type SgxQcnlResult<T> = result::Result<T, sgx_qcnl_error_t>;
pub type SgxQcnlError = result::Result<(), sgx_qcnl_error_t>;

pub type SysResult<T> = result::Result<T, sys_error_t>;
pub type SysError = result::Result<(), sys_error_t>;
