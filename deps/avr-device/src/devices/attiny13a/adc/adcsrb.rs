#[doc = "Register `ADCSRB` reader"]
pub struct R(crate::R<ADCSRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCSRB` writer"]
pub struct W(crate::W<ADCSRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCSRB_SPEC>;
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
impl From<crate::W<ADCSRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCSRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADTS` reader - ADC Auto Trigger Sources"]
pub type ADTS_R = crate::FieldReader<u8, ADTS_A>;
#[doc = "ADC Auto Trigger Sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTS_A {
    #[doc = "0: Free Running mode"]
    FREE = 0,
    #[doc = "1: Analog Comparator"]
    AC = 1,
    #[doc = "2: External Interrupt Request 0"]
    INT0 = 2,
    #[doc = "3: Timer/Counter0 Compare Match A"]
    TC0_CMA = 3,
    #[doc = "4: Timer/Counter0 Overflow"]
    TC0_OVF = 4,
    #[doc = "5: Timer/Counter0 Compare Match B"]
    TC0_CMB = 5,
    #[doc = "6: Pin Change Interrupt Request"]
    PCIR = 6,
}
impl From<ADTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTS_A) -> Self {
        variant as _
    }
}
impl ADTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADTS_A> {
        match self.bits {
            0 => Some(ADTS_A::FREE),
            1 => Some(ADTS_A::AC),
            2 => Some(ADTS_A::INT0),
            3 => Some(ADTS_A::TC0_CMA),
            4 => Some(ADTS_A::TC0_OVF),
            5 => Some(ADTS_A::TC0_CMB),
            6 => Some(ADTS_A::PCIR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == ADTS_A::FREE
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == ADTS_A::AC
    }
    #[doc = "Checks if the value of the field is `INT0`"]
    #[inline(always)]
    pub fn is_int0(&self) -> bool {
        *self == ADTS_A::INT0
    }
    #[doc = "Checks if the value of the field is `TC0_CMA`"]
    #[inline(always)]
    pub fn is_tc0_cma(&self) -> bool {
        *self == ADTS_A::TC0_CMA
    }
    #[doc = "Checks if the value of the field is `TC0_OVF`"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == ADTS_A::TC0_OVF
    }
    #[doc = "Checks if the value of the field is `TC0_CMB`"]
    #[inline(always)]
    pub fn is_tc0_cmb(&self) -> bool {
        *self == ADTS_A::TC0_CMB
    }
    #[doc = "Checks if the value of the field is `PCIR`"]
    #[inline(always)]
    pub fn is_pcir(&self) -> bool {
        *self == ADTS_A::PCIR
    }
}
#[doc = "Field `ADTS` writer - ADC Auto Trigger Sources"]
pub type ADTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADCSRB_SPEC, u8, ADTS_A, 3, O>;
impl<'a, const O: u8> ADTS_W<'a, O> {
    #[doc = "Free Running mode"]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(ADTS_A::FREE)
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut W {
        self.variant(ADTS_A::AC)
    }
    #[doc = "External Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(self) -> &'a mut W {
        self.variant(ADTS_A::INT0)
    }
    #[doc = "Timer/Counter0 Compare Match A"]
    #[inline(always)]
    pub fn tc0_cma(self) -> &'a mut W {
        self.variant(ADTS_A::TC0_CMA)
    }
    #[doc = "Timer/Counter0 Overflow"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut W {
        self.variant(ADTS_A::TC0_OVF)
    }
    #[doc = "Timer/Counter0 Compare Match B"]
    #[inline(always)]
    pub fn tc0_cmb(self) -> &'a mut W {
        self.variant(ADTS_A::TC0_CMB)
    }
    #[doc = "Pin Change Interrupt Request"]
    #[inline(always)]
    pub fn pcir(self) -> &'a mut W {
        self.variant(ADTS_A::PCIR)
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC Auto Trigger Sources"]
    #[inline(always)]
    pub fn adts(&self) -> ADTS_R {
        ADTS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Auto Trigger Sources"]
    #[inline(always)]
    #[must_use]
    pub fn adts(&mut self) -> ADTS_W<0> {
        ADTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control and Status Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcsrb](index.html) module"]
pub struct ADCSRB_SPEC;
impl crate::RegisterSpec for ADCSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcsrb::R](R) reader structure"]
impl crate::Readable for ADCSRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcsrb::W](W) writer structure"]
impl crate::Writable for ADCSRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSRB to value 0"]
impl crate::Resettable for ADCSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
