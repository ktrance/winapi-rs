// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Common dialog error return codes
// Done as of 10.0.14393.0
#![cfg(feature = "shared.cderr")]
use shared::minwindef::*;
pub const CDERR_DIALOGFAILURE: DWORD = 0xFFFF;
pub const CDERR_GENERALCODES: DWORD = 0x0000;
pub const CDERR_STRUCTSIZE: DWORD = 0x0001;
pub const CDERR_INITIALIZATION: DWORD = 0x0002;
pub const CDERR_NOTEMPLATE: DWORD = 0x0003;
pub const CDERR_NOHINSTANCE: DWORD = 0x0004;
pub const CDERR_LOADSTRFAILURE: DWORD = 0x0005;
pub const CDERR_FINDRESFAILURE: DWORD = 0x0006;
pub const CDERR_LOADRESFAILURE: DWORD = 0x0007;
pub const CDERR_LOCKRESFAILURE: DWORD = 0x0008;
pub const CDERR_MEMALLOCFAILURE: DWORD = 0x0009;
pub const CDERR_MEMLOCKFAILURE: DWORD = 0x000A;
pub const CDERR_NOHOOK: DWORD = 0x000B;
pub const CDERR_REGISTERMSGFAIL: DWORD = 0x000C;
pub const PDERR_PRINTERCODES: DWORD = 0x1000;
pub const PDERR_SETUPFAILURE: DWORD = 0x1001;
pub const PDERR_PARSEFAILURE: DWORD = 0x1002;
pub const PDERR_RETDEFFAILURE: DWORD = 0x1003;
pub const PDERR_LOADDRVFAILURE: DWORD = 0x1004;
pub const PDERR_GETDEVMODEFAIL: DWORD = 0x1005;
pub const PDERR_INITFAILURE: DWORD = 0x1006;
pub const PDERR_NODEVICES: DWORD = 0x1007;
pub const PDERR_NODEFAULTPRN: DWORD = 0x1008;
pub const PDERR_DNDMMISMATCH: DWORD = 0x1009;
pub const PDERR_CREATEICFAILURE: DWORD = 0x100A;
pub const PDERR_PRINTERNOTFOUND: DWORD = 0x100B;
pub const PDERR_DEFAULTDIFFERENT: DWORD = 0x100C;
pub const CFERR_CHOOSEFONTCODES: DWORD = 0x2000;
pub const CFERR_NOFONTS: DWORD = 0x2001;
pub const CFERR_MAXLESSTHANMIN: DWORD = 0x2002;
pub const FNERR_FILENAMECODES: DWORD = 0x3000;
pub const FNERR_SUBCLASSFAILURE: DWORD = 0x3001;
pub const FNERR_INVALIDFILENAME: DWORD = 0x3002;
pub const FNERR_BUFFERTOOSMALL: DWORD = 0x3003;
pub const FRERR_FINDREPLACECODES: DWORD = 0x4000;
pub const FRERR_BUFFERLENGTHZERO: DWORD = 0x4001;
pub const CCERR_CHOOSECOLORCODES: DWORD = 0x5000;
