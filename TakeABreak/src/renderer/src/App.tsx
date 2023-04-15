import { useEffect, useRef, useState } from 'react'
import Countdown from './components/Countdown'

function App(): JSX.Element {
  const [start, setStart] = useState(false)
  const [remainingTime, setRemainingTime] = useState(0)
  const [hours, setHours] = useState(0)
  const [minutes, setMinutes] = useState(0)
  const [seconds, setSeconds] = useState(0)

  const countdownRefId = useRef<ReturnType<typeof setInterval>>()

  // hatirlaticinin baslangic suresi.
  const deadlineRef = useRef(0)

  // zamanin her saniye -1 azalmasi.
  useEffect(() => {
    if (start && remainingTime > 0) {
      countdownRefId.current = setInterval(() => {
        setRemainingTime((prev) => prev - 1000)
      }, 1000)
    } else if (start && remainingTime <= 0) {
      // saniye 01 oldugunda aniden baslangic zamanina donmemesi icin ekstra 1 saniye daha bekliyoruz.
      // Bu sayede saniye 00 olduktan sonra baslangic zamanina doner.
      setTimeout(() => {
        setRemainingTime(deadlineRef.current)
      }, 1000)
    }
    // component unmount oldugunda interval objesinin de hafizadan ucurulmasi.
    return () => clearInterval(countdownRefId.current)
  }, [start, remainingTime])

  // hatirlaticinin baslangic suresinin hesaplanmasi.
  useEffect(() => {
    // saat, dakika veya saniye degistiginde zamani durdur.
    setStart(false)

    // zamanlayicinin sifirlanmasi icin gecemesi gereken toplam sure (milisaniye cinsinden).
    const deadline =
      (hours || 0) * 60 * 60 * 1000 + (minutes || 0) * 60 * 1000 + (seconds || 0) * 1000

    deadlineRef.current = deadline
    setRemainingTime(deadline)
  }, [hours, minutes, seconds])

  return (
    <div className="relative pb-20 pt-10 w-full h-full">
      {/* Header */}
      <div className="absolute inset-0 overflow-hidden bg-indigo-50 w-full h-full">
        <img
          src="/src/assets/background.jpg"
          alt="background-image"
          className="absolute left-0 top-0 translate-x-[-40%] translate-y-[20%] -scale-x-100"
        />
        <div className="absolute inset-x-0 top-0 h-40 bg-gradient-to-b from-white"></div>
        <div className="absolute inset-x-0 bottom-0 h-40 bg-gradient-to-t from-white"></div>
      </div>

      {/* Description */}
      <div className="mx-auto max-w-7xl px-4 relative">
        <h1 className="font-display text-5xl font-bold tracking-tighter text-blue-600">
          Take a Break
        </h1>
        <div className="mt-6 space-y-6 font-display text-2xl tracking-tight text-blue-900">
          <p>
            Spending long hours at the computer can affect your physical and mental health.
            Therefore, taking regular breaks is important.
          </p>
        </div>
      </div>

      {/* Countdown */}
      <Countdown remainingTime={remainingTime} />

      <div className="mx-auto max-w-7xl px-4 relative">
        <input
          type="number"
          value={hours}
          onChange={(e): void => setHours(parseInt(e.target.value))}
        />
        <input
          type="number"
          value={minutes}
          onChange={(e): void => setMinutes(parseInt(e.target.value))}
        />
        <input
          type="number"
          value={seconds}
          onChange={(e): void => setSeconds(parseInt(e.target.value))}
        />
        <button onClick={(): void => setStart(true)}>Start</button>
        <button onClick={(): void => setStart(false)}>Stop</button>
      </div>
      <div className="mx-auto max-w-7xl px-4 relative">{remainingTime}</div>
      <div className="mx-auto max-w-7xl px-4 relative">{deadlineRef.current}</div>
    </div>
  )
}
export default App
