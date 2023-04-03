import { type FC, type ReactNode, useState } from 'react'
import { Dots } from './components/Dots'
import { Menu } from './components/Menu'
import { type DensityValue, type DotValue, type StatusValue } from './type'
import { invoke } from '@tauri-apps/api/tauri'

export const App: FC = () => {
  const [status, setStatus] = useState<StatusValue>('init')
  const [w, setW] = useState(10)
  const [h, setH] = useState(10)
  const [density, setDensity] = useState<DensityValue>('middle')
  const [dots, setDots] = useState<DotValue[][]>([])

  const initGame: () => void = () => {
    invoke<{ dots: DotValue[][] }>('init_game', { w, h, density })
      .then((response) => {
        setDots(response.dots)
        setStatus('play')
      })
      .catch(console.log)
  }

  const getGame: () => ReactNode = () => {
    if (status === 'init') {
      return <button onClick={initGame}>Start</button>
    } else if (status === 'play') {
      return (
        <Dots
          dots={dots}
          setDots={setDots}
          setStatus={setStatus}
          disabled={false}
        />
      )
    } else if (status === 'bomb') {
      return (
        <>
          <Dots
            dots={dots}
            setDots={setDots}
            setStatus={setStatus}
            disabled={true}
          />
          <p>Game Over</p>
        </>
      )
    } else if (status === 'clear') {
      return (
        <>
          <Dots
            dots={dots}
            setDots={setDots}
            setStatus={setStatus}
            disabled={true}
          />
          <p>All Clear</p>
        </>
      )
    }
  }

  return (
    <div className="container">
      <Menu
        w={w}
        setW={setW}
        h={h}
        setH={setH}
        density={density}
        setDensity={setDensity}
        isPlaying={status !== 'init'}
        setStatus={setStatus}
      />
      {getGame()}
    </div>
  )
}
