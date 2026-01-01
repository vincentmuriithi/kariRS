#[doc = "Register `GPIOR0` reader"]
pub struct R(crate::R<GPIOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOR0` writer"]
pub struct W(crate::W<GPIOR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOR0_SPEC>;
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
impl From<crate::W<GPIOR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOR00` reader - General Purpose IO Register 0 bit 0"]
pub type GPIOR00_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR00` writer - General Purpose IO Register 0 bit 0"]
pub type GPIOR00_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
#[doc = "Field `GPIOR01` reader - General Purpose IO Register 0 bit 1"]
pub type GPIOR01_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR01` writer - General Purpose IO Register 0 bit 1"]
pub type GPIOR01_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
#[doc = "Field `GPIOR02` reader - General Purpose IO Register 0 bit 2"]
pub type GPIOR02_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR02` writer - General Purpose IO Register 0 bit 2"]
pub type GPIOR02_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
#[doc = "Field `GPIOR03` reader - General Purpose IO Register 0 bit 3"]
pub type GPIOR03_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR03` writer - General Purpose IO Register 0 bit 3"]
pub type GPIOR03_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
#[doc = "Field `GPIOR04` reader - General Purpose IO Register 0 bit 4"]
pub type GPIOR04_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR04` writer - General Purpose IO Register 0 bit 4"]
pub type GPIOR04_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
#[doc = "Field `GPIOR05` reader - General Purpose IO Register 0 bit 5"]
pub type GPIOR05_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR05` writer - General Purpose IO Register 0 bit 5"]
pub type GPIOR05_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
#[doc = "Field `GPIOR06` reader - General Purpose IO Register 0 bit 6"]
pub type GPIOR06_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR06` writer - General Purpose IO Register 0 bit 6"]
pub type GPIOR06_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
#[doc = "Field `GPIOR07` reader - General Purpose IO Register 0 bit 7"]
pub type GPIOR07_R = crate::BitReader<bool>;
#[doc = "Field `GPIOR07` writer - General Purpose IO Register 0 bit 7"]
pub type GPIOR07_W<'a, const O: u8> = crate::BitWriter<'a, u8, GPIOR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - General Purpose IO Register 0 bit 0"]
    #[inline(always)]
    pub fn gpior00(&self) -> GPIOR00_R {
        GPIOR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General Purpose IO Register 0 bit 1"]
    #[inline(always)]
    pub fn gpior01(&self) -> GPIOR01_R {
        GPIOR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Purpose IO Register 0 bit 2"]
    #[inline(always)]
    pub fn gpior02(&self) -> GPIOR02_R {
        GPIOR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Purpose IO Register 0 bit 3"]
    #[inline(always)]
    pub fn gpior03(&self) -> GPIOR03_R {
        GPIOR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Purpose IO Register 0 bit 4"]
    #[inline(always)]
    pub fn gpior04(&self) -> GPIOR04_R {
        GPIOR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Purpose IO Register 0 bit 5"]
    #[inline(always)]
    pub fn gpior05(&self) -> GPIOR05_R {
        GPIOR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Purpose IO Register 0 bit 6"]
    #[inline(always)]
    pub fn gpior06(&self) -> GPIOR06_R {
        GPIOR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - General Purpose IO Register 0 bit 7"]
    #[inline(always)]
    pub fn gpior07(&self) -> GPIOR07_R {
        GPIOR07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Purpose IO Register 0 bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpior00(&mut self) -> GPIOR00_W<0> {
        GPIOR00_W::new(self)
    }
    #[doc = "Bit 1 - General Purpose IO Register 0 bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpior01(&mut self) -> GPIOR01_W<1> {
        GPIOR01_W::new(self)
    }
    #[doc = "Bit 2 - General Purpose IO Register 0 bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpior02(&mut self) -> GPIOR02_W<2> {
        GPIOR02_W::new(self)
    }
    #[doc = "Bit 3 - General Purpose IO Register 0 bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpior03(&mut self) -> GPIOR03_W<3> {
        GPIOR03_W::new(self)
    }
    #[doc = "Bit 4 - General Purpose IO Register 0 bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpior04(&mut self) -> GPIOR04_W<4> {
        GPIOR04_W::new(self)
    }
    #[doc = "Bit 5 - General Purpose IO Register 0 bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpior05(&mut self) -> GPIOR05_W<5> {
        GPIOR05_W::new(self)
    }
    #[doc = "Bit 6 - General Purpose IO Register 0 bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gpior06(&mut self) -> GPIOR06_W<6> {
        GPIOR06_W::new(self)
    }
    #[doc = "Bit 7 - General Purpose IO Register 0 bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpior07(&mut self) -> GPIOR07_W<7> {
        GPIOR07_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose IO Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpior0](index.html) module"]
pub struct GPIOR0_SPEC;
impl crate::RegisterSpec for GPIOR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpior0::R](R) reader structure"]
impl crate::Readable for GPIOR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpior0::W](W) writer structure"]
impl crate::Writable for GPIOR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOR0 to value 0"]
impl crate::Resettable for GPIOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
