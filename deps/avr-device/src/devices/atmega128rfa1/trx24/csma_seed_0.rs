#[doc = "Register `CSMA_SEED_0` reader"]
pub struct R(crate::R<CSMA_SEED_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSMA_SEED_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSMA_SEED_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSMA_SEED_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSMA_SEED_0` writer"]
pub struct W(crate::W<CSMA_SEED_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSMA_SEED_0_SPEC>;
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
impl From<crate::W<CSMA_SEED_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSMA_SEED_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSMA_SEED_00` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_00_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_00` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_00_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
#[doc = "Field `CSMA_SEED_01` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_01_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_01` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_01_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
#[doc = "Field `CSMA_SEED_02` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_02_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_02` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_02_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
#[doc = "Field `CSMA_SEED_03` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_03_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_03` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_03_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
#[doc = "Field `CSMA_SEED_04` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_04_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_04` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_04_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
#[doc = "Field `CSMA_SEED_05` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_05_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_05` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_05_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
#[doc = "Field `CSMA_SEED_06` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_06_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_06` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_06_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
#[doc = "Field `CSMA_SEED_07` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_07_R = crate::BitReader<bool>;
#[doc = "Field `CSMA_SEED_07` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_07_W<'a, const O: u8> = crate::BitWriter<'a, u8, CSMA_SEED_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_00(&self) -> CSMA_SEED_00_R {
        CSMA_SEED_00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_01(&self) -> CSMA_SEED_01_R {
        CSMA_SEED_01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_02(&self) -> CSMA_SEED_02_R {
        CSMA_SEED_02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_03(&self) -> CSMA_SEED_03_R {
        CSMA_SEED_03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_04(&self) -> CSMA_SEED_04_R {
        CSMA_SEED_04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_05(&self) -> CSMA_SEED_05_R {
        CSMA_SEED_05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_06(&self) -> CSMA_SEED_06_R {
        CSMA_SEED_06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_07(&self) -> CSMA_SEED_07_R {
        CSMA_SEED_07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_00(&mut self) -> CSMA_SEED_00_W<0> {
        CSMA_SEED_00_W::new(self)
    }
    #[doc = "Bit 1 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_01(&mut self) -> CSMA_SEED_01_W<1> {
        CSMA_SEED_01_W::new(self)
    }
    #[doc = "Bit 2 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_02(&mut self) -> CSMA_SEED_02_W<2> {
        CSMA_SEED_02_W::new(self)
    }
    #[doc = "Bit 3 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_03(&mut self) -> CSMA_SEED_03_W<3> {
        CSMA_SEED_03_W::new(self)
    }
    #[doc = "Bit 4 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_04(&mut self) -> CSMA_SEED_04_W<4> {
        CSMA_SEED_04_W::new(self)
    }
    #[doc = "Bit 5 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_05(&mut self) -> CSMA_SEED_05_W<5> {
        CSMA_SEED_05_W::new(self)
    }
    #[doc = "Bit 6 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_06(&mut self) -> CSMA_SEED_06_W<6> {
        CSMA_SEED_06_W::new(self)
    }
    #[doc = "Bit 7 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_07(&mut self) -> CSMA_SEED_07_W<7> {
        CSMA_SEED_07_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver CSMA-CA Random Number Generator Seed Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csma_seed_0](index.html) module"]
pub struct CSMA_SEED_0_SPEC;
impl crate::RegisterSpec for CSMA_SEED_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [csma_seed_0::R](R) reader structure"]
impl crate::Readable for CSMA_SEED_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csma_seed_0::W](W) writer structure"]
impl crate::Writable for CSMA_SEED_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSMA_SEED_0 to value 0"]
impl crate::Resettable for CSMA_SEED_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
