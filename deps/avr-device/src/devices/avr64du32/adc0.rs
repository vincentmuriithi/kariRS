#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control C"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Control B"]
    pub ctrlc: CTRLC,
    #[doc = "0x03 - Control E"]
    pub ctrld: CTRLD,
    #[doc = "0x04 - Control F"]
    pub ctrle: CTRLE,
    #[doc = "0x05 - Control D"]
    pub ctrlf: CTRLF,
    #[doc = "0x06 - Interrupt Control"]
    pub intctrl: INTCTRL,
    #[doc = "0x07 - Interrupt Flags"]
    pub intflags: INTFLAGS,
    #[doc = "0x08 - Status"]
    pub status: STATUS,
    #[doc = "0x09 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x0a - Command"]
    pub command: COMMAND,
    #[doc = "0x0b - Positive mux input"]
    pub muxpos: MUXPOS,
    #[doc = "0x0c - ADC Accumulator Result"]
    pub result: RESULT,
    #[doc = "0x0e - ADC Sample"]
    pub sample: SAMPLE,
    #[doc = "0x10 - Window comparator low threshold"]
    pub winlt: WINLT,
    #[doc = "0x12 - Window comparator high threshold"]
    pub winht: WINHT,
    #[doc = "0x14 - Temporary Data"]
    pub temp: TEMP,
}
#[doc = "COMMAND (rw) register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command"]
pub mod command;
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control C"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control B"]
pub mod ctrlc;
#[doc = "CTRLD (rw) register accessor: an alias for `Reg<CTRLD_SPEC>`"]
pub type CTRLD = crate::Reg<ctrld::CTRLD_SPEC>;
#[doc = "Control E"]
pub mod ctrld;
#[doc = "CTRLE (rw) register accessor: an alias for `Reg<CTRLE_SPEC>`"]
pub type CTRLE = crate::Reg<ctrle::CTRLE_SPEC>;
#[doc = "Control F"]
pub mod ctrle;
#[doc = "CTRLF (rw) register accessor: an alias for `Reg<CTRLF_SPEC>`"]
pub type CTRLF = crate::Reg<ctrlf::CTRLF_SPEC>;
#[doc = "Control D"]
pub mod ctrlf;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "INTCTRL (rw) register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS (rw) register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "MUXPOS (rw) register accessor: an alias for `Reg<MUXPOS_SPEC>`"]
pub type MUXPOS = crate::Reg<muxpos::MUXPOS_SPEC>;
#[doc = "Positive mux input"]
pub mod muxpos;
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "ADC Accumulator Result"]
pub mod result;
#[doc = "SAMPLE (rw) register accessor: an alias for `Reg<SAMPLE_SPEC>`"]
pub type SAMPLE = crate::Reg<sample::SAMPLE_SPEC>;
#[doc = "ADC Sample"]
pub mod sample;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TEMP (rw) register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temporary Data"]
pub mod temp;
#[doc = "WINHT (rw) register accessor: an alias for `Reg<WINHT_SPEC>`"]
pub type WINHT = crate::Reg<winht::WINHT_SPEC>;
#[doc = "Window comparator high threshold"]
pub mod winht;
#[doc = "WINLT (rw) register accessor: an alias for `Reg<WINLT_SPEC>`"]
pub type WINLT = crate::Reg<winlt::WINLT_SPEC>;
#[doc = "Window comparator low threshold"]
pub mod winlt;
