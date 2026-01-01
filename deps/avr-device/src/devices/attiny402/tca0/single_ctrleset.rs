#[doc = "Register `CTRLESET` reader"]
pub struct R(crate::R<SINGLE_CTRLESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLE_CTRLESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLE_CTRLESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLE_CTRLESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLESET` writer"]
pub struct W(crate::W<SINGLE_CTRLESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLE_CTRLESET_SPEC>;
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
impl From<crate::W<SINGLE_CTRLESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLE_CTRLESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Count up"]
    UP = 0,
    #[doc = "1: Count down"]
    DOWN = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::UP,
            true => DIR_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DIR_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DIR_A::DOWN
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLESET_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Count up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(DIR_A::UP)
    }
    #[doc = "Count down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(DIR_A::DOWN)
    }
}
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LUPD_R = crate::BitReader<bool>;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LUPD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_CTRLESET_SPEC, bool, O>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No Command"]
    NONE = 0,
    #[doc = "1: Force Update"]
    UPDATE = 1,
    #[doc = "2: Force Restart"]
    RESTART = 2,
    #[doc = "3: Force Hard Reset"]
    RESET = 3,
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
    pub fn variant(&self) -> CMD_A {
        match self.bits {
            0 => CMD_A::NONE,
            1 => CMD_A::UPDATE,
            2 => CMD_A::RESTART,
            3 => CMD_A::RESET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMD_A::NONE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == CMD_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == CMD_A::RESTART
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CMD_A::RESET
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SINGLE_CTRLESET_SPEC, u8, CMD_A, 2, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "No Command"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMD_A::NONE)
    }
    #[doc = "Force Update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(CMD_A::UPDATE)
    }
    #[doc = "Force Restart"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(CMD_A::RESTART)
    }
    #[doc = "Force Hard Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CMD_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LUPD_W<1> {
        LUPD_W::new(self)
    }
    #[doc = "Bits 2:3 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<2> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control E Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_ctrleset](index.html) module"]
pub struct SINGLE_CTRLESET_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLESET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [single_ctrleset::R](R) reader structure"]
impl crate::Readable for SINGLE_CTRLESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [single_ctrleset::W](W) writer structure"]
impl crate::Writable for SINGLE_CTRLESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLESET to value 0"]
impl crate::Resettable for SINGLE_CTRLESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
