#[doc = "Register `TCAROUTEA` reader"]
pub struct R(crate::R<TCAROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCAROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCAROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCAROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCAROUTEA` writer"]
pub struct W(crate::W<TCAROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCAROUTEA_SPEC>;
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
impl From<crate::W<TCAROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCAROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCA0` reader - TCA0 Signals"]
pub type TCA0_R = crate::FieldReader<u8, TCA0_A>;
#[doc = "TCA0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCA0_A {
    #[doc = "0: WOn: PA0, PA1, PA2, PA3, PA4, PA5"]
    PORTA = 0,
    #[doc = "2: WOn: -, -, -, PC3, -, -"]
    PORTC = 2,
    #[doc = "3: WOn: PD0, PD1, PD2, PD3, PD4, PD5"]
    PORTD = 3,
    #[doc = "5: WOn: PF0, PF1, -, -, -, -"]
    PORTF = 5,
}
impl From<TCA0_A> for u8 {
    #[inline(always)]
    fn from(variant: TCA0_A) -> Self {
        variant as _
    }
}
impl TCA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCA0_A> {
        match self.bits {
            0 => Some(TCA0_A::PORTA),
            2 => Some(TCA0_A::PORTC),
            3 => Some(TCA0_A::PORTD),
            5 => Some(TCA0_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == TCA0_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == TCA0_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == TCA0_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == TCA0_A::PORTF
    }
}
#[doc = "Field `TCA0` writer - TCA0 Signals"]
pub type TCA0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCAROUTEA_SPEC, u8, TCA0_A, 3, O>;
impl<'a, const O: u8> TCA0_W<'a, O> {
    #[doc = "WOn: PA0, PA1, PA2, PA3, PA4, PA5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(TCA0_A::PORTA)
    }
    #[doc = "WOn: -, -, -, PC3, -, -"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(TCA0_A::PORTC)
    }
    #[doc = "WOn: PD0, PD1, PD2, PD3, PD4, PD5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(TCA0_A::PORTD)
    }
    #[doc = "WOn: PF0, PF1, -, -, -, -"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(TCA0_A::PORTF)
    }
}
impl R {
    #[doc = "Bits 0:2 - TCA0 Signals"]
    #[inline(always)]
    pub fn tca0(&self) -> TCA0_R {
        TCA0_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - TCA0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn tca0(&mut self) -> TCA0_W<0> {
        TCA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCA route A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcaroutea](index.html) module"]
pub struct TCAROUTEA_SPEC;
impl crate::RegisterSpec for TCAROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcaroutea::R](R) reader structure"]
impl crate::Readable for TCAROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcaroutea::W](W) writer structure"]
impl crate::Writable for TCAROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCAROUTEA to value 0"]
impl crate::Resettable for TCAROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
