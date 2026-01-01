#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPCODEWP` reader - Application Code Write Protect"]
pub type APPCODEWP_R = crate::BitReader<bool>;
#[doc = "Field `APPCODEWP` writer - Application Code Write Protect"]
pub type APPCODEWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `BOOTRP` reader - Boot Read Protect"]
pub type BOOTRP_R = crate::BitReader<bool>;
#[doc = "Field `BOOTRP` writer - Boot Read Protect"]
pub type BOOTRP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `APPDATAWP` reader - Application Data Write Protect"]
pub type APPDATAWP_R = crate::BitReader<bool>;
#[doc = "Field `APPDATAWP` writer - Application Data Write Protect"]
pub type APPDATAWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `EEWP` reader - EEPROM Write Protect"]
pub type EEWP_R = crate::BitReader<bool>;
#[doc = "Field `EEWP` writer - EEPROM Write Protect"]
pub type EEWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `FLMAP` reader - Flash Mapping in Data space"]
pub type FLMAP_R = crate::FieldReader<u8, FLMAP_A>;
#[doc = "Flash Mapping in Data space\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLMAP_A {
    #[doc = "0: Flash section 0"]
    SECTION0 = 0,
    #[doc = "1: Flash section 1"]
    SECTION1 = 1,
    #[doc = "2: Flash section 2"]
    SECTION2 = 2,
    #[doc = "3: Flash section 3"]
    SECTION3 = 3,
}
impl From<FLMAP_A> for u8 {
    #[inline(always)]
    fn from(variant: FLMAP_A) -> Self {
        variant as _
    }
}
impl FLMAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLMAP_A {
        match self.bits {
            0 => FLMAP_A::SECTION0,
            1 => FLMAP_A::SECTION1,
            2 => FLMAP_A::SECTION2,
            3 => FLMAP_A::SECTION3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SECTION0`"]
    #[inline(always)]
    pub fn is_section0(&self) -> bool {
        *self == FLMAP_A::SECTION0
    }
    #[doc = "Checks if the value of the field is `SECTION1`"]
    #[inline(always)]
    pub fn is_section1(&self) -> bool {
        *self == FLMAP_A::SECTION1
    }
    #[doc = "Checks if the value of the field is `SECTION2`"]
    #[inline(always)]
    pub fn is_section2(&self) -> bool {
        *self == FLMAP_A::SECTION2
    }
    #[doc = "Checks if the value of the field is `SECTION3`"]
    #[inline(always)]
    pub fn is_section3(&self) -> bool {
        *self == FLMAP_A::SECTION3
    }
}
#[doc = "Field `FLMAP` writer - Flash Mapping in Data space"]
pub type FLMAP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLB_SPEC, u8, FLMAP_A, 2, O>;
impl<'a, const O: u8> FLMAP_W<'a, O> {
    #[doc = "Flash section 0"]
    #[inline(always)]
    pub fn section0(self) -> &'a mut W {
        self.variant(FLMAP_A::SECTION0)
    }
    #[doc = "Flash section 1"]
    #[inline(always)]
    pub fn section1(self) -> &'a mut W {
        self.variant(FLMAP_A::SECTION1)
    }
    #[doc = "Flash section 2"]
    #[inline(always)]
    pub fn section2(self) -> &'a mut W {
        self.variant(FLMAP_A::SECTION2)
    }
    #[doc = "Flash section 3"]
    #[inline(always)]
    pub fn section3(self) -> &'a mut W {
        self.variant(FLMAP_A::SECTION3)
    }
}
#[doc = "Field `FLMAPLOCK` reader - Flash Mapping Lock"]
pub type FLMAPLOCK_R = crate::BitReader<bool>;
#[doc = "Field `FLMAPLOCK` writer - Flash Mapping Lock"]
pub type FLMAPLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Application Code Write Protect"]
    #[inline(always)]
    pub fn appcodewp(&self) -> APPCODEWP_R {
        APPCODEWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot Read Protect"]
    #[inline(always)]
    pub fn bootrp(&self) -> BOOTRP_R {
        BOOTRP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Application Data Write Protect"]
    #[inline(always)]
    pub fn appdatawp(&self) -> APPDATAWP_R {
        APPDATAWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EEPROM Write Protect"]
    #[inline(always)]
    pub fn eewp(&self) -> EEWP_R {
        EEWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Flash Mapping in Data space"]
    #[inline(always)]
    pub fn flmap(&self) -> FLMAP_R {
        FLMAP_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Flash Mapping Lock"]
    #[inline(always)]
    pub fn flmaplock(&self) -> FLMAPLOCK_R {
        FLMAPLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Application Code Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn appcodewp(&mut self) -> APPCODEWP_W<0> {
        APPCODEWP_W::new(self)
    }
    #[doc = "Bit 1 - Boot Read Protect"]
    #[inline(always)]
    #[must_use]
    pub fn bootrp(&mut self) -> BOOTRP_W<1> {
        BOOTRP_W::new(self)
    }
    #[doc = "Bit 2 - Application Data Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn appdatawp(&mut self) -> APPDATAWP_W<2> {
        APPDATAWP_W::new(self)
    }
    #[doc = "Bit 3 - EEPROM Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn eewp(&mut self) -> EEWP_W<3> {
        EEWP_W::new(self)
    }
    #[doc = "Bits 4:5 - Flash Mapping in Data space"]
    #[inline(always)]
    #[must_use]
    pub fn flmap(&mut self) -> FLMAP_W<4> {
        FLMAP_W::new(self)
    }
    #[doc = "Bit 7 - Flash Mapping Lock"]
    #[inline(always)]
    #[must_use]
    pub fn flmaplock(&mut self) -> FLMAPLOCK_W<7> {
        FLMAPLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
