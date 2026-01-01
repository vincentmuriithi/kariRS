#[doc = "Register `CTRLD` reader"]
pub struct R(crate::R<CTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLD` writer"]
pub struct W(crate::W<CTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLD_SPEC>;
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
impl From<crate::W<CTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPDLY` reader - Sampling Delay Selection"]
pub type SAMPDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPDLY` writer - Sampling Delay Selection"]
pub type SAMPDLY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLD_SPEC, u8, u8, 4, O>;
#[doc = "Field `ASDV` reader - Automatic Sampling Delay Variation"]
pub type ASDV_R = crate::BitReader<ASDV_A>;
#[doc = "Automatic Sampling Delay Variation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASDV_A {
    #[doc = "0: The Automatic Sampling Delay Variation is disabled"]
    ASVOFF = 0,
    #[doc = "1: The Automatic Sampling Delay Variation is enabled"]
    ASVON = 1,
}
impl From<ASDV_A> for bool {
    #[inline(always)]
    fn from(variant: ASDV_A) -> Self {
        variant as u8 != 0
    }
}
impl ASDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASDV_A {
        match self.bits {
            false => ASDV_A::ASVOFF,
            true => ASDV_A::ASVON,
        }
    }
    #[doc = "Checks if the value of the field is `ASVOFF`"]
    #[inline(always)]
    pub fn is_asvoff(&self) -> bool {
        *self == ASDV_A::ASVOFF
    }
    #[doc = "Checks if the value of the field is `ASVON`"]
    #[inline(always)]
    pub fn is_asvon(&self) -> bool {
        *self == ASDV_A::ASVON
    }
}
#[doc = "Field `ASDV` writer - Automatic Sampling Delay Variation"]
pub type ASDV_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, ASDV_A, O>;
impl<'a, const O: u8> ASDV_W<'a, O> {
    #[doc = "The Automatic Sampling Delay Variation is disabled"]
    #[inline(always)]
    pub fn asvoff(self) -> &'a mut W {
        self.variant(ASDV_A::ASVOFF)
    }
    #[doc = "The Automatic Sampling Delay Variation is enabled"]
    #[inline(always)]
    pub fn asvon(self) -> &'a mut W {
        self.variant(ASDV_A::ASVON)
    }
}
#[doc = "Field `INITDLY` reader - Initial Delay Selection"]
pub type INITDLY_R = crate::FieldReader<u8, INITDLY_A>;
#[doc = "Initial Delay Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INITDLY_A {
    #[doc = "0: Delay 0 CLK_ADC cycles"]
    DLY0 = 0,
    #[doc = "1: Delay 16 CLK_ADC cycles"]
    DLY16 = 1,
    #[doc = "2: Delay 32 CLK_ADC cycles"]
    DLY32 = 2,
    #[doc = "3: Delay 64 CLK_ADC cycles"]
    DLY64 = 3,
    #[doc = "4: Delay 128 CLK_ADC cycles"]
    DLY128 = 4,
    #[doc = "5: Delay 256 CLK_ADC cycles"]
    DLY256 = 5,
}
impl From<INITDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: INITDLY_A) -> Self {
        variant as _
    }
}
impl INITDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INITDLY_A> {
        match self.bits {
            0 => Some(INITDLY_A::DLY0),
            1 => Some(INITDLY_A::DLY16),
            2 => Some(INITDLY_A::DLY32),
            3 => Some(INITDLY_A::DLY64),
            4 => Some(INITDLY_A::DLY128),
            5 => Some(INITDLY_A::DLY256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DLY0`"]
    #[inline(always)]
    pub fn is_dly0(&self) -> bool {
        *self == INITDLY_A::DLY0
    }
    #[doc = "Checks if the value of the field is `DLY16`"]
    #[inline(always)]
    pub fn is_dly16(&self) -> bool {
        *self == INITDLY_A::DLY16
    }
    #[doc = "Checks if the value of the field is `DLY32`"]
    #[inline(always)]
    pub fn is_dly32(&self) -> bool {
        *self == INITDLY_A::DLY32
    }
    #[doc = "Checks if the value of the field is `DLY64`"]
    #[inline(always)]
    pub fn is_dly64(&self) -> bool {
        *self == INITDLY_A::DLY64
    }
    #[doc = "Checks if the value of the field is `DLY128`"]
    #[inline(always)]
    pub fn is_dly128(&self) -> bool {
        *self == INITDLY_A::DLY128
    }
    #[doc = "Checks if the value of the field is `DLY256`"]
    #[inline(always)]
    pub fn is_dly256(&self) -> bool {
        *self == INITDLY_A::DLY256
    }
}
#[doc = "Field `INITDLY` writer - Initial Delay Selection"]
pub type INITDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLD_SPEC, u8, INITDLY_A, 3, O>;
impl<'a, const O: u8> INITDLY_W<'a, O> {
    #[doc = "Delay 0 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly0(self) -> &'a mut W {
        self.variant(INITDLY_A::DLY0)
    }
    #[doc = "Delay 16 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly16(self) -> &'a mut W {
        self.variant(INITDLY_A::DLY16)
    }
    #[doc = "Delay 32 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly32(self) -> &'a mut W {
        self.variant(INITDLY_A::DLY32)
    }
    #[doc = "Delay 64 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly64(self) -> &'a mut W {
        self.variant(INITDLY_A::DLY64)
    }
    #[doc = "Delay 128 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly128(self) -> &'a mut W {
        self.variant(INITDLY_A::DLY128)
    }
    #[doc = "Delay 256 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly256(self) -> &'a mut W {
        self.variant(INITDLY_A::DLY256)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sampling Delay Selection"]
    #[inline(always)]
    pub fn sampdly(&self) -> SAMPDLY_R {
        SAMPDLY_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Automatic Sampling Delay Variation"]
    #[inline(always)]
    pub fn asdv(&self) -> ASDV_R {
        ASDV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Initial Delay Selection"]
    #[inline(always)]
    pub fn initdly(&self) -> INITDLY_R {
        INITDLY_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sampling Delay Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sampdly(&mut self) -> SAMPDLY_W<0> {
        SAMPDLY_W::new(self)
    }
    #[doc = "Bit 4 - Automatic Sampling Delay Variation"]
    #[inline(always)]
    #[must_use]
    pub fn asdv(&mut self) -> ASDV_W<4> {
        ASDV_W::new(self)
    }
    #[doc = "Bits 5:7 - Initial Delay Selection"]
    #[inline(always)]
    #[must_use]
    pub fn initdly(&mut self) -> INITDLY_W<5> {
        INITDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrld](index.html) module"]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrld::R](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrld::W](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
