import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { isMobile as _isMobile } from 'react-device-detect'

// Stitches
import { styled } from '../../../../stitches.config'

// Logos
import IconClock from 'logos/clock.svg'
import Logo from 'logos/header-logo.svg'
import Button from 'components/primitives/button'
import { ArrowDown } from 'components/primitives/arrow'
import Container from '../container'
import Box from 'components/common/box'
import MobileMenu from './mobile-menu'
import { useToggleState } from 'hooks/use-toggle-state'
import { useLocomotiveScroll } from 'context/locomotive-scroll'
import { useRouter } from 'next/router'
import { event } from 'lib/ga'
import { download } from '../../../lib/utils'

const StyledHeader = styled('header', {
  my: '$4',
  position: 'fixed',
  left: 0,
  top: 0,
  width: '100%',
  zIndex: '9998'
})

const StyledTime = styled('time', {
  display: 'inline-flex',
  justifyContent: 'flex-start',
  textTransform: 'uppercase',
  width: '100px',
  '> span': {
    display: 'inline-flex',
    justifyContent: 'space-between',
    flexGrow: 1
  }
})

const Time = ({ variant }: { variant?: 'mobile' }) => {
  const [now, setNow] = useState(new Date())

  useEffect(() => {
    const interval = window.setInterval(() => {
      setNow(new Date())
    }, 1000)

    return () => {
      window.clearInterval(interval)
    }
  }, [])

  const renderTime = useCallback((date: Date) => {
    let hours: number | string = date.getHours()
    let minutes: number | string = date.getMinutes()
    let seconds: number | string = date.getSeconds()
    const isAm = hours <= 12
    hours = hours % 12
    hours = hours ? hours : 12 // the hour '0' should be '12'
    hours = hours < 10 ? '0' + hours : hours
    minutes = minutes < 10 ? '0' + minutes : minutes
    seconds = seconds < 10 ? '0' + seconds : seconds
    return (
      <StyledTime>
        <span>
          {hours}:{minutes}:{seconds} <span>{isAm ? 'AM' : 'PM'}</span>
        </span>
      </StyledTime>
    )
  }, [])

  return (
    <Box
      css={{
        display: 'flex',
        alignItems: 'center',
        px: '$$px',
        height: '100%',
        borderLeft: '1px solid black',
        borderRight: '1px solid black',
        ...(variant === 'mobile'
          ? {
              flexGrow: 1,
              justifyContent: 'center'
            }
          : undefined)
      }}
    >
      <IconClock style={{ marginRight: 8 }} />
      {renderTime(now)}
    </Box>
  )
}

const StyledButton = styled(Button, {
  fontWeight: '700',
  textTransform: 'uppercase'
})

export const DownloadButton = ({
  variant,
  className,
  tabIndex
}: {
  variant?: 'mobile'
  className?: string
  tabIndex?: number
}) => {
  const [isMobile, setIsMobile] = useState<boolean>()

  useEffect(() => {
    setIsMobile(_isMobile)
  }, [])

  const shouldOnlyTweet = useMemo(() => {
    return isMobile || variant === 'mobile'
  }, [isMobile, variant])

  const handleDownload = useCallback(() => {
    event({
      category: 'Download',
      action: 'Download',
      label: 'Download',
      value: '1'
    })
    const encoded = {
      text: encodeURIComponent(
        "Yangi o'zimizning o'zbek dasturchilarimiz tomonidan ishlab chiqilgan dasturlash tili chiqdi. Dasturlash tili Osmon tili deb ataladi va uni shu sahifadan ko'rishingiz mumkin!"
      ),
      url: encodeURIComponent('https://osmon.dev')
    }
    window.open(
      `https://t.me/share/url?url=${encoded.url}&text=${encoded.text}`,
      '_blank'
    )
    if (!shouldOnlyTweet) {
      switch (window.navigator.platform) {
        case 'Win32':
        case 'Win64':
          download(
            encodeURI(location.origin + '/osmon-x86_64-pc-windows-msvc.zip')
          )
          break
        case 'MacIntel':
          download(
            encodeURI(location.origin + '/osmon-x86_64-apple-darwin.zip')
          )
          break
        case 'Linux x86_64':
          download(
            encodeURI(location.origin + '/osmon-x86_64-unknown-linux-gnu.zip')
          )
          break
        default:
      }
    }
  }, [shouldOnlyTweet])

  return (
    <StyledButton
      onClick={handleDownload}
      className={className}
      tabIndex={tabIndex}
      css={{
        px: '$$px',
        ...(variant === 'mobile'
          ? {
              fontSize: '40px',
              textAlign: 'left',
              position: 'relative'
            }
          : { height: '100%' })
      }}
      icon={
        <ArrowDown
          // @ts-ignore
          css={{
            $$size: '15px',
            transform: 'rotate(270deg)',
            ...(variant === 'mobile'
              ? {
                  position: 'absolute',
                  $$size: '32px',
                  bottom: '14px',
                  left: '200px'
                }
              : undefined)
          }}
        />
      }
    >
      {shouldOnlyTweet ? 'ULASHING' : <>ULASHING VA &nbsp;YUKLANG</>}{' '}
    </StyledButton>
  )
}

const Header = ({
  platforms
}: {
  platforms: { arch: string; status: string }[]
}) => {
  const router = useRouter()
  const mobileMenuState = useToggleState()
  const { scroll } = useLocomotiveScroll()

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth > 519) {
        mobileMenuState.handleOff()
      }
    }

    window.addEventListener('resize', handleResize)
    return () => {
      window.removeEventListener('resize', handleResize)
    }
  }, [mobileMenuState])

  const handleLogoClick = useCallback(() => {
    if (scroll) {
      scroll.scrollTo(0)
    }
  }, [scroll])

  const handleContextMenu: React.MouseEventHandler<HTMLAnchorElement> =
    useCallback(
      (e) => {
        e.preventDefault()
        router.push('https://osmon.dev')
      },
      [router]
    )

  return (
    <StyledHeader id="header">
      <Container
        css={{
          fontWeight: '700',
          color: 'white'
        }}
        maxWidth
      >
        <Box
          css={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            border: '1px solid black',
            height: '56px',
            backdropFilter: 'saturate(180%) blur(5px)',
            backgroundColor: 'rgba(16, 16, 16, .8)',

            $$px: '18px'
          }}
        >
          <Box css={{ display: 'flex', alignItems: 'center', height: '100%' }}>
            <Link href="/" passHref>
              <Box
                as="a"
                title="Basement Grotesque"
                css={{
                  px: '$$px',
                  height: '100%',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center'
                }}
                onClick={handleLogoClick}
                onContextMenu={handleContextMenu}
              >
                <Logo style={{ margin: 0 }} />
              </Box>
            </Link>
            <Box
              css={{
                display: 'none',
                alignItems: 'center',
                borderLeft: '1px solid black',
                borderRight: '1px solid black',
                px: '$$px',
                height: '100%',
                '.divider': { mx: '12px' },
                '.regular': { fontWeight: '400' },
                '> p, > span': {
                  display: 'none'
                },
                '.disabled': { color: 'rgba(255,255,255,0.5)' },

                '@media screen and (min-width: 742px)': {
                  display: 'flex',
                  '.ipad': {
                    display: 'block'
                  }
                },

                '@media screen and (min-width: 1268px)': {
                  '> p, > span': {
                    display: 'block'
                  }
                }
              }}
            >
              <p className="ipad">
                <span>{platforms[0].arch}</span> <span>/</span>{' '}
                <span className="regular">{platforms[0].status}</span>
              </p>
              <span className="divider">·</span>
              <p className="disabled">
                <span>{platforms[1].arch}</span> <span>/</span>{' '}
                <span className="regular">{platforms[1].status}</span>
              </p>
              <span className="divider">·</span>
              <p className="ipad">
                <span>{platforms[2].arch}</span> <span>/</span>{' '}
                <span className="regular">{platforms[2].status}</span>
              </p>
            </Box>
          </Box>
          <Box
            css={{
              display: 'none',
              '@bp1': {
                display: 'flex'
              },
              alignItems: 'center',
              height: '100%',
              flexShrink: 0
            }}
          >
            <Time />
            <DownloadButton />
          </Box>
          <Box
            css={{
              height: '100%',
              display: 'flex',
              alignItems: 'center',
              flexGrow: 1,
              '@bp1': { display: 'none' }
            }}
          >
            <Time variant="mobile" />
          </Box>
          <Box
            css={{
              height: '100%',
              display: 'flex',
              alignItems: 'center',
              '@bp1': { display: 'none' }
            }}
          >
            <MobileMenu {...mobileMenuState}>
              <p className="stagger">
                <span>{platforms[0].arch}</span> <span>/</span>{' '}
                <span className="regular">{platforms[0].status}</span>
              </p>
              <p className="disabled stagger">
                <span>{platforms[1].arch}</span> <span>/</span>{' '}
                <span className="regular">{platforms[1].status}</span>
              </p>
              <p className="stagger">
                <span>{platforms[2].arch}</span> <span>/</span>{' '}
                <span className="regular">{platforms[2].status}</span>
              </p>
            </MobileMenu>
          </Box>
        </Box>
      </Container>
    </StyledHeader>
  )
}

export default Header
