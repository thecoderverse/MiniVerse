import React, { useEffect, useRef, useState } from 'react'

function App(): JSX.Element {
  const [start, setStart] = useState(false)
  const [deadline, setDeadline] = useState(0)
  const [hours, setHours] = useState(0)
  const [minutes, setMinutes] = useState(0)
  const [seconds, setSeconds] = useState(0)

  const countdownRefId = useRef<ReturnType<typeof setInterval>>()

  useEffect(() => {
    if (start) {
      countdownRefId.current = setInterval(() => {
        setDeadline((prev) => prev - 1000)
      }, 1000)
    }
    return () => clearInterval(countdownRefId.current)
  }, [start, deadline])

  useEffect(() => {
    setDeadline((hours || 0) * 60 * 60 * 1000 + (minutes || 0) * 60 * 1000 + (seconds || 0) * 1000)
  }, [hours, minutes, seconds])

  const hoursChangeHandler = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setStart(false)
    setHours(parseInt(e.target.value))
  }

  const minutesChangeHandler = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setStart(false)
    setMinutes(parseInt(e.target.value))
  }

  const secondsChangeHandler = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setStart(false)
    setSeconds(parseInt(e.target.value))
  }

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
      <div className="max-w-7xl px-4 my-8 tracking-tight text-blue-900 relative text-center">
        <span className="countdown font-mono text-6xl">
          <span
            style={{ '--value': Math.floor((deadline % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60)) }}
          ></span>
          :
          <span
            style={{ '--value': Math.floor((deadline % (1000 * 60 * 60)) / (1000 * 60)) }}
          ></span>
          :<span style={{ '--value': Math.floor((deadline % (1000 * 60)) / 1000) }}></span>
        </span>
      </div>

      <div className="mx-auto max-w-7xl px-4 relative">
        <input type="number" value={hours} onChange={hoursChangeHandler} />
        <input type="number" value={minutes} onChange={minutesChangeHandler} />
        <input type="number" value={seconds} onChange={secondsChangeHandler} />
        <button onClick={(): void => setStart(true)}>Start</button>
        <button onClick={(): void => setStart(false)}>Stop</button>
      </div>
      <div className="mx-auto max-w-7xl px-4 relative">{deadline}</div>
    </div>
  )
}

export default App
