#[doc = "Register `PAN_ID_0` reader"]
pub struct R(crate::R<PAN_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAN_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAN_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAN_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAN_ID_0` writer"]
pub struct W(crate::W<PAN_ID_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAN_ID_0_SPEC>;
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
impl From<crate::W<PAN_ID_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAN_ID_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAN_ID_00` reader - MAC Personal Area Network ID"]
pub type PAN_ID_00_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_00` writer - MAC Personal Area Network ID"]
pub type PAN_ID_00_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
#[doc = "Field `PAN_ID_01` reader - MAC Personal Area Network ID"]
pub type PAN_ID_01_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_01` writer - MAC Personal Area Network ID"]
pub type PAN_ID_01_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
#[doc = "Field `PAN_ID_02` reader - MAC Personal Area Network ID"]
pub type PAN_ID_02_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_02` writer - MAC Personal Area Network ID"]
pub type PAN_ID_02_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
#[doc = "Field `PAN_ID_03` reader - MAC Personal Area Network ID"]
pub type PAN_ID_03_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_03` writer - MAC Personal Area Network ID"]
pub type PAN_ID_03_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
#[doc = "Field `PAN_ID_04` reader - MAC Personal Area Network ID"]
pub type PAN_ID_04_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_04` writer - MAC Personal Area Network ID"]
pub type PAN_ID_04_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
#[doc = "Field `PAN_ID_05` reader - MAC Personal Area Network ID"]
pub type PAN_ID_05_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_05` writer - MAC Personal Area Network ID"]
pub type PAN_ID_05_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
#[doc = "Field `PAN_ID_06` reader - MAC Personal Area Network ID"]
pub type PAN_ID_06_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_06` writer - MAC Personal Area Network ID"]
pub type PAN_ID_06_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
#[doc = "Field `PAN_ID_07` reader - MAC Personal Area Network ID"]
pub type PAN_ID_07_R = crate::BitReader<bool>;
#[doc = "Field `PAN_ID_07` writer - MAC Personal Area Network ID"]
pub type PAN_ID_07_W<'a, const O: u8> = crate::BitWriter<'a, u8, PAN_ID_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_00(&self) -> PAN_ID_00_R {
        PAN_ID_00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_01(&self) -> PAN_ID_01_R {
        PAN_ID_01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_02(&self) -> PAN_ID_02_R {
        PAN_ID_02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_03(&self) -> PAN_ID_03_R {
        PAN_ID_03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_04(&self) -> PAN_ID_04_R {
        PAN_ID_04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_05(&self) -> PAN_ID_05_R {
        PAN_ID_05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_06(&self) -> PAN_ID_06_R {
        PAN_ID_06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_07(&self) -> PAN_ID_07_R {
        PAN_ID_07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_00(&mut self) -> PAN_ID_00_W<0> {
        PAN_ID_00_W::new(self)
    }
    #[doc = "Bit 1 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_01(&mut self) -> PAN_ID_01_W<1> {
        PAN_ID_01_W::new(self)
    }
    #[doc = "Bit 2 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_02(&mut self) -> PAN_ID_02_W<2> {
        PAN_ID_02_W::new(self)
    }
    #[doc = "Bit 3 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_03(&mut self) -> PAN_ID_03_W<3> {
        PAN_ID_03_W::new(self)
    }
    #[doc = "Bit 4 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_04(&mut self) -> PAN_ID_04_W<4> {
        PAN_ID_04_W::new(self)
    }
    #[doc = "Bit 5 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_05(&mut self) -> PAN_ID_05_W<5> {
        PAN_ID_05_W::new(self)
    }
    #[doc = "Bit 6 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_06(&mut self) -> PAN_ID_06_W<6> {
        PAN_ID_06_W::new(self)
    }
    #[doc = "Bit 7 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_07(&mut self) -> PAN_ID_07_W<7> {
        PAN_ID_07_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Personal Area Network ID Register (Low Byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pan_id_0](index.html) module"]
pub struct PAN_ID_0_SPEC;
impl crate::RegisterSpec for PAN_ID_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pan_id_0::R](R) reader structure"]
impl crate::Readable for PAN_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pan_id_0::W](W) writer structure"]
impl crate::Writable for PAN_ID_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAN_ID_0 to value 0"]
impl crate::Resettable for PAN_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
