import { useEffect } from 'react'
import { useInView } from 'react-intersection-observer'

// Primitives
import Box from 'components/common/box'
import Container from 'components/layout/container'
import Section from 'components/layout/section'

// Styles
import { styled } from '../../../../stitches.config'
import AbAnimation from './ab-animation'
import { DURATION, gsap, SplitText } from 'lib/gsap'

const Text = styled('p', {
  fontFamily: '$body',
  fontSize: 'clamp($3, 1.35vw, $4)',
  lineHeight: 'clamp(25px, 1.5vw, 22px)',
  fontWeight: 500,
  b: {
    fontFamily: '$heading'
  },

  variants: {
    size: {
      bg: {
        fontSize: 'clamp($4, 2vw, $7)',
        lineHeight: 'clamp($5, 2.2vw, $8)'
      },
      sm: {
        fontSize: '$2',
        lineHeight: '$4',
        '@bp2': {
          fontSize: '$3',
          lineHeight: '$5'
        }
      }
    }
  },
  length: 0
})

const AboutSection = () => {
  const { ref, inView } = useInView({ triggerOnce: true, threshold: 0.2 })

  useEffect(() => {
    gsap.set('#about-section', {
      autoAlpha: 0
    })
  }, [])

  useEffect(() => {
    if (!inView) return

    const title = new SplitText('.about__title', {
      type: 'lines,words,chars'
    })

    const subtitle = new SplitText('.about__subtitle', {
      type: 'lines'
    })

    const tl = gsap.timeline({
      paused: true,
      smoothChildTiming: true
    })
    tl.to('#about-section', {
      autoAlpha: 1,
      duration: DURATION / 2
    })
    tl.in(title.lines)
    tl.in(subtitle.lines, '<40%')

    tl.timeScale(1.3).play()

    return () => {
      tl.kill()
    }
  }, [inView])

  const uwu = 'https://uwussi.moe/'
  const chat = 'https://t.me/osmonchat'

  return (
    <Section
      // @ts-ignore
      background="black"
      css={{ pt: '128px' }}
      id="about-section"
    >
      <Container
        // @ts-ignore
        autoPy
        css={{ pb: 0 }}
        maxWidth
        ref={ref}
      >
        <Box
          css={{
            display: 'grid',
            gridTemplateColumns: '1fr',
            gap: '64px',
            '@bp2': {
              gridTemplateColumns: '1fr 1fr'
            }
          }}
        >
          <Box
            data-scroll-speed={0.6}
            data-scroll
            css={{
              maxWidth: 708,
              gridRowStart: 2,
              '@bp2': { gridRowStart: 'initial' }
            }}
          >
            <Text
              className="about__title"
              // @ts-ignore
              size="bg"
              css={{
                marginBottom: 48,
                '@bp2': { marginBottom: 28 }
              }}
            >
              <b>osmon tili</b> bu o'zbek dasturchilar tomonidan yaratilgan
              birinchi to'liq dasturlash tilidir. Bu tilning yaralishidan
              maqsad, dasturchilikga endi kirib kelayotgan, ammo til yoki boshqa
              turdagi muammolar sababli o'rganolmayotganlar uchun yechimdir.
            </Text>
            <Text
              className="about__subtitle"
              css={{
                mb: 16
              }}
            >
              Bu dasturlash tili no'ldan qurilgan va o'zbek auditoriyasi uchun
              to'laqonlikcha moslashtirilgandir. Dasturlash tili hozirgi eng
              xavfsiz va tez dasturlash tili hisoblanmish, Rust daturlash tili
              yordamida yozilgan. Til esa dasturni ishga tushurish uchun o'zinig
              shaxsiy "bulut" nomli virtual mashinasini ishlatadi.
            </Text>
            <Text className="about__subtitle">
              Dasturlash tili asosan{' '}
              <a href={uwu}>
                <u>
                  <b>uwussimo</b>
                </u>
              </a>{' '}
              tomonidan yaratilgan va shu dasturchi kuzatuvi ostida
              shakllantirilib boriladi. Agar sizda quyidagi proyekt bo'yicha
              savol yoki muammolaringiz bo'lsa, iltimos, osmon dasturlash tili
              jamiyati{' '}
              <a href={chat}>
                <u>
                  <b>chatiga</b>
                </u>
              </a>{' '}
              murojaat qiling!
            </Text>
          </Box>
          <Box
            css={{
              display: 'flex',
              alignItems: 'center',
              '@bp2': { margin: '-128px -128px -128px -32px' }
            }}
          >
            <AbAnimation />
          </Box>
        </Box>
        <Box
          data-scroll-speed={0.1}
          data-scroll
          css={{
            ta: 'center',
            fontWeight: 800,
            fontFamily: '$heading',
            fontSize: 'clamp($4, 2vw, $7)',
            lineHeight: 'clamp($5, 2.2vw, $8)',
            mt: '64px',
            '@bp2': {
              mt: '128px'
            }
          }}
        >
          ***
        </Box>
      </Container>
    </Section>
  )
}

export default AboutSection
