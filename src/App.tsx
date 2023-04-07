import { type FC, useState } from 'react'
import { Dots } from './components/Dots'
import { Menu } from './components/Menu'
import {
  type DensityValue,
  type DotValue,
  type SizeValue,
  type StatusValue,
} from './type'

export const App: FC = () => {
  const [status, setStatus] = useState<StatusValue>('init')
  const [size, setSize] = useState<SizeValue>('small')
  const [density, setDensity] = useState<DensityValue>('middle')
  const [dots, setDots] = useState<DotValue[][]>([])

  return (
    <div className="container">
      <Menu
        setStatus={setStatus}
        size={size}
        setSize={setSize}
        density={density}
        setDensity={setDensity}
        setDots={setDots}
        status={status}
      />
      {status !== 'init' ? (
        <Dots
          dots={dots}
          setDots={setDots}
          setStatus={setStatus}
          isPlaying={status === 'play'}
        />
      ) : (
        <></>
      )}
    </div>
  )
}
