import { type FC, useState } from 'react'
import { Dots } from './components/Dots'
import { Menu } from './components/Menu'
import { type DensityValue, type DotValue, type StateValue } from './type'
import { invoke } from '@tauri-apps/api/tauri'

export const App: FC = () => {
  const [state, setState] = useState<StateValue>('init')
  const [w, setW] = useState(10)
  const [h, setH] = useState(10)
  const [density, setDensity] = useState<DensityValue>('middle')
  const [dots, setDots] = useState<DotValue[][]>([])

  const initDots: () => void = () => {
    invoke<{ dots: DotValue[][] }>('init_dots', { w, h })
      .then((response) => {
        setDots(response.dots)
        setState('play')
      })
      .catch(() => {})
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
      />
      {state === 'init' ? (
        <button
          onClick={() => {
            initDots()
          }}
        >
          Start
        </button>
      ) : (
        <></>
      )}
      {state === 'play' ? <Dots dots={dots} setDots={setDots} /> : <></>}
    </div>
  )
}
