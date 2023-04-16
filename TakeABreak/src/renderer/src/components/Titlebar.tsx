import { IoMdClose } from 'react-icons/io'
const Titlebar = (): JSX.Element => {
  return (
    <div className="titlebar p-3 px-4 relative bg-transparent flex justify-end">
      <button className="text-2xl text-blue-900">
        <IoMdClose />
      </button>
    </div>
  )
}

export default Titlebar
