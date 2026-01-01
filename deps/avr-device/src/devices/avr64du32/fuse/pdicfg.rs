#[doc = "Register `PDICFG` reader"]
pub struct R(crate::R<PDICFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDICFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDICFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDICFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDICFG` writer"]
pub struct W(crate::W<PDICFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDICFG_SPEC>;
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
impl From<crate::W<PDICFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDICFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVEL` reader - Protection Level"]
pub type LEVEL_R = crate::FieldReader<u8, LEVEL_A>;
#[doc = "Protection Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEVEL_A {
    #[doc = "1: NVM Access through UPDI disabled"]
    NVMACCDIS = 1,
    #[doc = "3: UPDI and UPDI pins working normally"]
    BASIC = 3,
}
impl From<LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEVEL_A) -> Self {
        variant as _
    }
}
impl LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEVEL_A> {
        match self.bits {
            1 => Some(LEVEL_A::NVMACCDIS),
            3 => Some(LEVEL_A::BASIC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NVMACCDIS`"]
    #[inline(always)]
    pub fn is_nvmaccdis(&self) -> bool {
        *self == LEVEL_A::NVMACCDIS
    }
    #[doc = "Checks if the value of the field is `BASIC`"]
    #[inline(always)]
    pub fn is_basic(&self) -> bool {
        *self == LEVEL_A::BASIC
    }
}
#[doc = "Field `LEVEL` writer - Protection Level"]
pub type LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDICFG_SPEC, u8, LEVEL_A, 2, O>;
impl<'a, const O: u8> LEVEL_W<'a, O> {
    #[doc = "NVM Access through UPDI disabled"]
    #[inline(always)]
    pub fn nvmaccdis(self) -> &'a mut W {
        self.variant(LEVEL_A::NVMACCDIS)
    }
    #[doc = "UPDI and UPDI pins working normally"]
    #[inline(always)]
    pub fn basic(self) -> &'a mut W {
        self.variant(LEVEL_A::BASIC)
    }
}
#[doc = "Field `KEY` reader - NVM Protection Activation Key"]
pub type KEY_R = crate::FieldReader<u16, KEY_A>;
#[doc = "NVM Protection Activation Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum KEY_A {
    #[doc = "0: Not Active"]
    NOTACT = 0,
    #[doc = "2885: NVM Protection Active"]
    NVMACT = 2885,
}
impl From<KEY_A> for u16 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            0 => Some(KEY_A::NOTACT),
            2885 => Some(KEY_A::NVMACT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACT`"]
    #[inline(always)]
    pub fn is_notact(&self) -> bool {
        *self == KEY_A::NOTACT
    }
    #[doc = "Checks if the value of the field is `NVMACT`"]
    #[inline(always)]
    pub fn is_nvmact(&self) -> bool {
        *self == KEY_A::NVMACT
    }
}
#[doc = "Field `KEY` writer - NVM Protection Activation Key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDICFG_SPEC, u16, KEY_A, 12, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Not Active"]
    #[inline(always)]
    pub fn notact(self) -> &'a mut W {
        self.variant(KEY_A::NOTACT)
    }
    #[doc = "NVM Protection Active"]
    #[inline(always)]
    pub fn nvmact(self) -> &'a mut W {
        self.variant(KEY_A::NVMACT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Protection Level"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:15 - NVM Protection Activation Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits >> 4) & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Protection Level"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<0> {
        LEVEL_W::new(self)
    }
    #[doc = "Bits 4:15 - NVM Protection Activation Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<4> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programming and Debugging Interface Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdicfg](index.html) module"]
pub struct PDICFG_SPEC;
impl crate::RegisterSpec for PDICFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdicfg::R](R) reader structure"]
impl crate::Readable for PDICFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdicfg::W](W) writer structure"]
impl crate::Writable for PDICFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDICFG to value 0"]
impl crate::Resettable for PDICFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
