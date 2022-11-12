import React, { useEffect, useRef, useState } from 'react'
import { useInView } from 'react-intersection-observer'

// Primitives
import Section from 'components/layout/section'

// Styles
import { styled } from '../../../../stitches.config'
import Box from 'components/common/box'
import Container from 'components/layout/container'
import { DURATION, gsap, SplitText } from 'lib/gsap'

const SectionInner = styled(Container, {
  color: '$white',
  pt: '128px',
  position: 'relative'
})

const CharactersSection = () => {
  const { ref, inView } = useInView({ triggerOnce: true, threshold: 0.2 })
  // @ts-ignore
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [viewAll, setViewAll] = useState(false)
  // @ts-ignore
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [gridHeight, setGridHeight] = useState<number>()
  const gridRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handleResize = () => {
      if (!gridRef.current) return
      setGridHeight(gridRef.current.offsetHeight)
    }

    handleResize()
    window.addEventListener('resize', handleResize)
    return () => {
      window.removeEventListener('resize', handleResize)
    }
  }, [])

  useEffect(() => {
    gsap.set('#characters-section', {
      autoAlpha: 0
    })
  }, [])

  useEffect(() => {
    if (!inView) return

    const title = new SplitText('.characters__title', {
      type: 'lines,words,chars'
    })

    const tl = gsap.timeline({
      paused: true,
      smoothChildTiming: true
    })
    tl.to('#characters-section', {
      autoAlpha: 1,
      duration: DURATION / 2
    })
    tl.in(title.chars)
    tl.from(
      '.characters__svg',
      {
        autoAlpha: 0,
        y: 30
      },
      '< 80%'
    )

    tl.timeScale(1.5).play()

    return () => {
      tl.kill()
    }
  }, [inView])

  return (
    <Section background="black">
      <SectionInner
        id="characters-section"
        autoPy
        css={{ pb: viewAll ? '128px' : '0px' }}
        maxWidth
        ref={ref}
      >
        <Box
          css={{
            position: 'relative',
            py: '64px',
            mb: '64px',
            '@bp2': {
              py: '128px',
              mb: '128px'
            }
          }}
        >
          <Box
            css={{
              position: 'absolute',
              pointerEvents: 'none',
              left: '50%',
              top: '50%',
              transform: 'translate(-50%, -50%)'
            }}
          >
            <svg
              className="characters__svg"
              width="638"
              height="810"
              viewBox="0 0 638 810"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M528.721 68.2758C543.566 77.5248 551.979 95.6945 554.372 120.803C556.765 145.901 553.132 177.835 543.988 214.45C525.702 287.673 485.406 379.5 427.425 472.562C369.443 565.624 304.776 642.267 247.108 690.953C218.272 715.299 191.208 732.635 167.624 741.549C144.031 750.466 124.013 750.923 109.168 741.674C94.3235 732.425 85.9114 714.255 83.5175 689.147C81.1245 664.049 84.758 632.115 93.9019 595.5C112.188 522.277 152.484 430.45 210.465 337.388C268.447 244.326 333.114 167.683 390.782 118.997C419.618 94.6513 446.682 77.3151 470.266 68.4012C493.859 59.4839 513.877 59.0269 528.721 68.2758Z"
                stroke="white"
              />
            </svg>
          </Box>
          <Box
            as="p"
            className="characters__title"
            css={{
              fontFamily: '$heading',
              fontSize: '48px',
              fontWeight: 800,
              wordBreak: 'break-all',
              textAlign: 'center',
              lineHeight: '1',
              maxWidth: '1280px',
              margin: 'auto',

              '@bp2': {
                fontSize: '88px'
              }
            }}
            data-scroll-speed={-0.6}
            data-scroll
          >
            texnologiya kelajagi
            <br />
            bizning qo'limizda
            <br />
            va u shu yerdan boshlanadi!
          </Box>
        </Box>
      </SectionInner>
    </Section>
  )
}

export default CharactersSection
