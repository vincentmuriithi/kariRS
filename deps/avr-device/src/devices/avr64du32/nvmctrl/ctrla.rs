#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No Command"]
    NONE = 0,
    #[doc = "1: No Operation"]
    NOOP = 1,
    #[doc = "2: Flash Write"]
    FLWR = 2,
    #[doc = "8: Flash Page Erase"]
    FLPER = 8,
    #[doc = "9: Flash Multi-Page Erase 2 pages"]
    FLMPER2 = 9,
    #[doc = "10: Flash Multi-Page Erase 4 pages"]
    FLMPER4 = 10,
    #[doc = "11: Flash Multi-Page Erase 8 pages"]
    FLMPER8 = 11,
    #[doc = "12: Flash Multi-Page Erase 16 pages"]
    FLMPER16 = 12,
    #[doc = "13: Flash Multi-Page Erase 32 pages"]
    FLMPER32 = 13,
    #[doc = "18: EEPROM Write"]
    EEWR = 18,
    #[doc = "19: EEPROM Erase and Write"]
    EEERWR = 19,
    #[doc = "24: EEPROM Byte Erase"]
    EEBER = 24,
    #[doc = "25: EEPROM Multi-Byte Erase 2 bytes"]
    EEMBER2 = 25,
    #[doc = "26: EEPROM Multi-Byte Erase 4 bytes"]
    EEMBER4 = 26,
    #[doc = "27: EEPROM Multi-Byte Erase 8 bytes"]
    EEMBER8 = 27,
    #[doc = "28: EEPROM Multi-Byte Erase 16 bytes"]
    EEMBER16 = 28,
    #[doc = "29: EEPROM Multi-Byte Erase 32 bytes"]
    EEMBER32 = 29,
    #[doc = "32: Chip Erase Command"]
    CHER = 32,
    #[doc = "48: EEPROM Erase Command"]
    EECHER = 48,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::NONE),
            1 => Some(CMD_A::NOOP),
            2 => Some(CMD_A::FLWR),
            8 => Some(CMD_A::FLPER),
            9 => Some(CMD_A::FLMPER2),
            10 => Some(CMD_A::FLMPER4),
            11 => Some(CMD_A::FLMPER8),
            12 => Some(CMD_A::FLMPER16),
            13 => Some(CMD_A::FLMPER32),
            18 => Some(CMD_A::EEWR),
            19 => Some(CMD_A::EEERWR),
            24 => Some(CMD_A::EEBER),
            25 => Some(CMD_A::EEMBER2),
            26 => Some(CMD_A::EEMBER4),
            27 => Some(CMD_A::EEMBER8),
            28 => Some(CMD_A::EEMBER16),
            29 => Some(CMD_A::EEMBER32),
            32 => Some(CMD_A::CHER),
            48 => Some(CMD_A::EECHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMD_A::NONE
    }
    #[doc = "Checks if the value of the field is `NOOP`"]
    #[inline(always)]
    pub fn is_noop(&self) -> bool {
        *self == CMD_A::NOOP
    }
    #[doc = "Checks if the value of the field is `FLWR`"]
    #[inline(always)]
    pub fn is_flwr(&self) -> bool {
        *self == CMD_A::FLWR
    }
    #[doc = "Checks if the value of the field is `FLPER`"]
    #[inline(always)]
    pub fn is_flper(&self) -> bool {
        *self == CMD_A::FLPER
    }
    #[doc = "Checks if the value of the field is `FLMPER2`"]
    #[inline(always)]
    pub fn is_flmper2(&self) -> bool {
        *self == CMD_A::FLMPER2
    }
    #[doc = "Checks if the value of the field is `FLMPER4`"]
    #[inline(always)]
    pub fn is_flmper4(&self) -> bool {
        *self == CMD_A::FLMPER4
    }
    #[doc = "Checks if the value of the field is `FLMPER8`"]
    #[inline(always)]
    pub fn is_flmper8(&self) -> bool {
        *self == CMD_A::FLMPER8
    }
    #[doc = "Checks if the value of the field is `FLMPER16`"]
    #[inline(always)]
    pub fn is_flmper16(&self) -> bool {
        *self == CMD_A::FLMPER16
    }
    #[doc = "Checks if the value of the field is `FLMPER32`"]
    #[inline(always)]
    pub fn is_flmper32(&self) -> bool {
        *self == CMD_A::FLMPER32
    }
    #[doc = "Checks if the value of the field is `EEWR`"]
    #[inline(always)]
    pub fn is_eewr(&self) -> bool {
        *self == CMD_A::EEWR
    }
    #[doc = "Checks if the value of the field is `EEERWR`"]
    #[inline(always)]
    pub fn is_eeerwr(&self) -> bool {
        *self == CMD_A::EEERWR
    }
    #[doc = "Checks if the value of the field is `EEBER`"]
    #[inline(always)]
    pub fn is_eeber(&self) -> bool {
        *self == CMD_A::EEBER
    }
    #[doc = "Checks if the value of the field is `EEMBER2`"]
    #[inline(always)]
    pub fn is_eember2(&self) -> bool {
        *self == CMD_A::EEMBER2
    }
    #[doc = "Checks if the value of the field is `EEMBER4`"]
    #[inline(always)]
    pub fn is_eember4(&self) -> bool {
        *self == CMD_A::EEMBER4
    }
    #[doc = "Checks if the value of the field is `EEMBER8`"]
    #[inline(always)]
    pub fn is_eember8(&self) -> bool {
        *self == CMD_A::EEMBER8
    }
    #[doc = "Checks if the value of the field is `EEMBER16`"]
    #[inline(always)]
    pub fn is_eember16(&self) -> bool {
        *self == CMD_A::EEMBER16
    }
    #[doc = "Checks if the value of the field is `EEMBER32`"]
    #[inline(always)]
    pub fn is_eember32(&self) -> bool {
        *self == CMD_A::EEMBER32
    }
    #[doc = "Checks if the value of the field is `CHER`"]
    #[inline(always)]
    pub fn is_cher(&self) -> bool {
        *self == CMD_A::CHER
    }
    #[doc = "Checks if the value of the field is `EECHER`"]
    #[inline(always)]
    pub fn is_eecher(&self) -> bool {
        *self == CMD_A::EECHER
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, CMD_A, 7, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "No Command"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMD_A::NONE)
    }
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn noop(self) -> &'a mut W {
        self.variant(CMD_A::NOOP)
    }
    #[doc = "Flash Write"]
    #[inline(always)]
    pub fn flwr(self) -> &'a mut W {
        self.variant(CMD_A::FLWR)
    }
    #[doc = "Flash Page Erase"]
    #[inline(always)]
    pub fn flper(self) -> &'a mut W {
        self.variant(CMD_A::FLPER)
    }
    #[doc = "Flash Multi-Page Erase 2 pages"]
    #[inline(always)]
    pub fn flmper2(self) -> &'a mut W {
        self.variant(CMD_A::FLMPER2)
    }
    #[doc = "Flash Multi-Page Erase 4 pages"]
    #[inline(always)]
    pub fn flmper4(self) -> &'a mut W {
        self.variant(CMD_A::FLMPER4)
    }
    #[doc = "Flash Multi-Page Erase 8 pages"]
    #[inline(always)]
    pub fn flmper8(self) -> &'a mut W {
        self.variant(CMD_A::FLMPER8)
    }
    #[doc = "Flash Multi-Page Erase 16 pages"]
    #[inline(always)]
    pub fn flmper16(self) -> &'a mut W {
        self.variant(CMD_A::FLMPER16)
    }
    #[doc = "Flash Multi-Page Erase 32 pages"]
    #[inline(always)]
    pub fn flmper32(self) -> &'a mut W {
        self.variant(CMD_A::FLMPER32)
    }
    #[doc = "EEPROM Write"]
    #[inline(always)]
    pub fn eewr(self) -> &'a mut W {
        self.variant(CMD_A::EEWR)
    }
    #[doc = "EEPROM Erase and Write"]
    #[inline(always)]
    pub fn eeerwr(self) -> &'a mut W {
        self.variant(CMD_A::EEERWR)
    }
    #[doc = "EEPROM Byte Erase"]
    #[inline(always)]
    pub fn eeber(self) -> &'a mut W {
        self.variant(CMD_A::EEBER)
    }
    #[doc = "EEPROM Multi-Byte Erase 2 bytes"]
    #[inline(always)]
    pub fn eember2(self) -> &'a mut W {
        self.variant(CMD_A::EEMBER2)
    }
    #[doc = "EEPROM Multi-Byte Erase 4 bytes"]
    #[inline(always)]
    pub fn eember4(self) -> &'a mut W {
        self.variant(CMD_A::EEMBER4)
    }
    #[doc = "EEPROM Multi-Byte Erase 8 bytes"]
    #[inline(always)]
    pub fn eember8(self) -> &'a mut W {
        self.variant(CMD_A::EEMBER8)
    }
    #[doc = "EEPROM Multi-Byte Erase 16 bytes"]
    #[inline(always)]
    pub fn eember16(self) -> &'a mut W {
        self.variant(CMD_A::EEMBER16)
    }
    #[doc = "EEPROM Multi-Byte Erase 32 bytes"]
    #[inline(always)]
    pub fn eember32(self) -> &'a mut W {
        self.variant(CMD_A::EEMBER32)
    }
    #[doc = "Chip Erase Command"]
    #[inline(always)]
    pub fn cher(self) -> &'a mut W {
        self.variant(CMD_A::CHER)
    }
    #[doc = "EEPROM Erase Command"]
    #[inline(always)]
    pub fn eecher(self) -> &'a mut W {
        self.variant(CMD_A::EECHER)
    }
}
impl R {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
