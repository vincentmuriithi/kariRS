#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EEBUSY` reader - EEPROM busy"]
pub type EEBUSY_R = crate::BitReader<bool>;
#[doc = "Field `FLBUSY` reader - Flash busy"]
pub type FLBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` reader - Write error"]
pub type ERROR_R = crate::FieldReader<u8, ERROR_A>;
#[doc = "Write error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERROR_A {
    #[doc = "0: No Error"]
    NOERROR = 0,
    #[doc = "1: Write command not selected or not valid"]
    INVALIDCMD = 1,
    #[doc = "2: Write to section not allowed"]
    WRITEPROTECT = 2,
    #[doc = "3: Selecting new write command while programming is ongoing"]
    CMDCOLLISION = 3,
}
impl From<ERROR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as _
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERROR_A> {
        match self.bits {
            0 => Some(ERROR_A::NOERROR),
            1 => Some(ERROR_A::INVALIDCMD),
            2 => Some(ERROR_A::WRITEPROTECT),
            3 => Some(ERROR_A::CMDCOLLISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == ERROR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `INVALIDCMD`"]
    #[inline(always)]
    pub fn is_invalidcmd(&self) -> bool {
        *self == ERROR_A::INVALIDCMD
    }
    #[doc = "Checks if the value of the field is `WRITEPROTECT`"]
    #[inline(always)]
    pub fn is_writeprotect(&self) -> bool {
        *self == ERROR_A::WRITEPROTECT
    }
    #[doc = "Checks if the value of the field is `CMDCOLLISION`"]
    #[inline(always)]
    pub fn is_cmdcollision(&self) -> bool {
        *self == ERROR_A::CMDCOLLISION
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM busy"]
    #[inline(always)]
    pub fn eebusy(&self) -> EEBUSY_R {
        EEBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash busy"]
    #[inline(always)]
    pub fn flbusy(&self) -> FLBUSY_R {
        FLBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Write error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
