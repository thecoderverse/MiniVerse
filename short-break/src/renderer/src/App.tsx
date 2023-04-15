function App(): JSX.Element {
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
      <div className="max-w-7xl px-4 my-8 text-8xl tracking-tight text-blue-900 relative text-center">
        01:55:24
      </div>
    </div>
  )
}

export default App
