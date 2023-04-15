type CountdownProps = {
  remainingTime: number
}

const Countdown = ({ remainingTime }: CountdownProps): JSX.Element => {
  return (
    <div className="max-w-7xl px-4 my-8 tracking-tight text-blue-900 relative text-center">
      <span className="countdown text-7xl">
        <span
          style={{
            '--value': Math.floor((remainingTime % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))
          }}
        ></span>
        :
        <span
          style={{ '--value': Math.floor((remainingTime % (1000 * 60 * 60)) / (1000 * 60)) }}
        ></span>
        :<span style={{ '--value': Math.floor((remainingTime % (1000 * 60)) / 1000) }}></span>
      </span>
    </div>
  )
}

export default Countdown
