// Copyright 2024 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1-or-later

#[derive(ValidateStream, ParseStream, Default)]
#[repr(C, packed)]
struct FuStructSynapticsVmm9 {
    signature: [char; 7] == "CARRERA",
}

#[repr(u8)]
enum FuSynapticsVmm9RcCtrl {
    EnableRc = 0x01,
    DisableRc = 0x02,
    GetId = 0x03,
    EraseFlash = 0x14,
    ActivateFirmware = 0x18,
    WriteFlashData = 0x20,
    MemoryWrite = 0x21,
    TxDpcdRegisterWrite = 0x22, // TX0 to TX3
    ReadFlashData = 0x30,
    MemoryRead = 0x31,
    TxDpcdRegisterRead = 0x32, // TX0 to TX3

    // these are fake, but useful for debugging
    EnableRcBusy = 0x80|0x01,
    DisableRcBusy = 0x80|0x02,
    GetIdBusy = 0x80|0x03,
    EraseFlashBusy = 0x80|0x14,
    ActivateFirmwareBusy = 0x80|0x18,
    WriteFlashDataBusy = 0x80|0x20,
    MemoryWriteBusy = 0x80|0x21,
    ReadFlashDataBusy = 0x80|0x30,
    MemoryReadBusy = 0x80|0x31,
}

#[derive(ToString)]
#[repr(u8)]
enum FuSynapticsVmm9RcSts {
    Success,
    Invalid,
    Unsupported,
    Failed,
    Disabled,
    ConfigureSignFailed,
    FirmwareSignFailed,
    RollbackFailed,
}

#[derive(New, Getters)]
#[repr(C, packed)]
struct FuStructHidPayload {
    cap: u8,
    state: u8,
    ctrl: FuSynapticsVmm9RcCtrl,
    sts: FuSynapticsVmm9RcSts,
    offset: u32le,
    length: u32le,
    fifo: [u8; 32],
}

#[derive(New, ToString, Getters, Default)]
#[repr(C, packed)]
struct FuStructHidSetCommand {
    id: u8 == 0x1,
    type: u8 == 0x0, // packet write
    size: u8,
    payload: FuStructHidPayload,
    checksum: u8, // this is actually lower if @rc_fifo is less than 32 bytes
}

#[derive(New, Parse, Default)]
#[repr(C, packed)]
struct FuStructHidGetCommand {
    id: u8 == 0x1,
    type: u8 == 0x0, // packet reply
    size: u8,
    payload: FuStructHidPayload,
    checksum: u8, // payload is always 32 bytes
}

#[derive(Parse)]
#[repr(C, packed)]
struct FuStructSynapticsUpdGetId {
    _pid: u16le,
    cid: u8,
    bid: u8,
}
