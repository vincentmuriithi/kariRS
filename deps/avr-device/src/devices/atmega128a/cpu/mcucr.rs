#[doc = "Register `MCUCR` reader"]
pub struct R(crate::R<MCUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCR` writer"]
pub struct W(crate::W<MCUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCR_SPEC>;
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
impl From<crate::W<MCUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVCE` reader - Interrupt Vector Change Enable"]
pub type IVCE_R = crate::BitReader<bool>;
#[doc = "Field `IVCE` writer - Interrupt Vector Change Enable"]
pub type IVCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader<bool>;
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
#[doc = "Field `SM2` reader - Sleep Mode Select"]
pub type SM2_R = crate::BitReader<SM2_A>;
#[doc = "Sleep Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM2_A {
    #[doc = "0: Idle"]
    IDLE = 0,
}
impl From<SM2_A> for bool {
    #[inline(always)]
    fn from(variant: SM2_A) -> Self {
        variant as u8 != 0
    }
}
impl SM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SM2_A> {
        match self.bits {
            false => Some(SM2_A::IDLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SM2_A::IDLE
    }
}
#[doc = "Field `SM2` writer - Sleep Mode Select"]
pub type SM2_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, SM2_A, O>;
impl<'a, const O: u8> SM2_W<'a, O> {
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(SM2_A::IDLE)
    }
}
#[doc = "Field `SM` reader - Sleep Mode Select"]
pub type SM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SM` writer - Sleep Mode Select"]
pub type SM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MCUCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SE` reader - Sleep Enable"]
pub type SE_R = crate::BitReader<bool>;
#[doc = "Field `SE` writer - Sleep Enable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
#[doc = "Field `SRW10` reader - External SRAM Wait State Select"]
pub type SRW10_R = crate::BitReader<bool>;
#[doc = "Field `SRW10` writer - External SRAM Wait State Select"]
pub type SRW10_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
#[doc = "Field `SRE` reader - External SRAM Enable"]
pub type SRE_R = crate::BitReader<bool>;
#[doc = "Field `SRE` writer - External SRAM Enable"]
pub type SRE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Vector Change Enable"]
    #[inline(always)]
    pub fn ivce(&self) -> IVCE_R {
        IVCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Vector Select"]
    #[inline(always)]
    pub fn ivsel(&self) -> IVSEL_R {
        IVSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep Mode Select"]
    #[inline(always)]
    pub fn sm2(&self) -> SM2_R {
        SM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Sleep Mode Select"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External SRAM Wait State Select"]
    #[inline(always)]
    pub fn srw10(&self) -> SRW10_R {
        SRW10_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External SRAM Enable"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Vector Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ivce(&mut self) -> IVCE_W<0> {
        IVCE_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivsel(&mut self) -> IVSEL_W<1> {
        IVSEL_W::new(self)
    }
    #[doc = "Bit 2 - Sleep Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm2(&mut self) -> SM2_W<2> {
        SM2_W::new(self)
    }
    #[doc = "Bits 3:4 - Sleep Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<3> {
        SM_W::new(self)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<5> {
        SE_W::new(self)
    }
    #[doc = "Bit 6 - External SRAM Wait State Select"]
    #[inline(always)]
    #[must_use]
    pub fn srw10(&mut self) -> SRW10_W<6> {
        SRW10_W::new(self)
    }
    #[doc = "Bit 7 - External SRAM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SRE_W<7> {
        SRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucr](index.html) module"]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcucr::R](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucr::W](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
