import { useEffect, useRef, useState } from 'react'
import Countdown from './components/Countdown'

import { FaPause, FaPlay } from 'react-icons/fa'
import Titlebar from './components/Titlebar'

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

  const func = async (): Promise<void> => {
    // await window.api.addNewWindow()
  }

  return (
    <div className="relative pb-20 h-full font-mono">
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

      <Titlebar />

      {/* Description */}
      <div className="mx-auto p-4 relative">
        <h1 className="text-5xl font-bold tracking-tighter text-blue-600">Take a Break</h1>
        <div className="mt-6 space-y-6 text-xl tracking-tight text-blue-900">
          <p>
            Spending long hours at the computer can affect your physical and mental health.
            Therefore, taking regular breaks is important.
          </p>
        </div>
      </div>

      {/* Countdown */}
      <Countdown remainingTime={remainingTime} />

      <div className="text-blue-900 mx-auto px-4 relative flex justify-center">
        <div className="flex flex-col items-center ">
          <label className="text-xl text-blue-800 ">H</label>
          <input
            className="shadow-lg shadow-blue-500/50 bg-transparent border-r-0 py-3 w-20 text-center text-2xl font-bold rounded-s-md focus:outline focus:outline-blue-500 outline-offset-2 outline-2"
            type="number"
            placeholder="H"
            value={hours}
            onChange={(e): void =>
              setHours(parseInt(e.target.value) > 23 ? 23 : parseInt(e.target.value))
            }
          />
        </div>
        <div className="flex flex-col items-center ">
          <label className="text-xl text-blue-800 ">M</label>
          <input
            className="shadow-lg shadow-blue-500/50 bg-transparent border-1 py-3 w-20 text-center text-2xl font-bold  focus:outline focus:outline-blue-500 outline-offset-2 outline-2"
            type="number"
            placeholder="M"
            value={minutes}
            onChange={(e): void =>
              setMinutes(parseInt(e.target.value) > 59 ? 59 : parseInt(e.target.value))
            }
          />
        </div>
        <div className="flex flex-col items-center ">
          <label className="text-xl text-blue-800 ">S</label>
          <input
            className="shadow-xl shadow-blue-500/50 bg-transparent border-l-0 py-3 w-20 text-center text-2xl font-bold rounded-r-md  focus:outline focus:outline-blue-500 outline-offset-2 outline-2"
            placeholder="S"
            type="number"
            value={seconds}
            onChange={(e): void =>
              setSeconds(parseInt(e.target.value) > 59 ? 59 : parseInt(e.target.value))
            }
          />
        </div>
      </div>
      <div className="relative flex justify-center mt-12">
        {start ? (
          <button className="p-2" onClick={(): void => setStart(false)}>
            <FaPause className="text-blue-900 w-8 h-8" />
          </button>
        ) : (
          <button className="p-2" onClick={(): void => setStart(true)}>
            <FaPlay className="text-blue-900 w-8 h-8" />
          </button>
        )}
        {/* <button onClick={func}>new window</button> */}
      </div>
    </div>
  )
}
export default App
