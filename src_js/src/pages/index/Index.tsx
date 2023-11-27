import { useState } from "react"
import { useNavigate } from "react-router-dom"

const Index = () => {
    const [version, setVersion] = useState('0.5.2')
    const [build, setBuild] = useState('2023-11-21')
    const navigate = useNavigate()

    const gotoInstall =() => {
        navigate('/installing')
    }

    return (
        <>
            <h1 className="text-center text-2xl">Welcome to OpenDAN, your personal AIOS</h1>
            <div className="flex mt-10 justify-center gap-4 text-dan-green">
                <a href="">OpenDAN Repo</a>
                <a href="">OpenDAN DAO Page</a>
            </div>

            <div className="flex items-center mt-20 gap-6">
                <span>My folder</span>
                <input className="flex-1 border rounded-lg h-9 px-4" />
                <div className="btn-dan w-24 h-9">choose</div>
            </div>

            <div className="mt-10 flex-center">
                <input className="w-4 h-4" type="checkbox"/>

                <div className="ml-2">
                    Agree OpenDAN Desktop User Agreements
                </div>
            </div>

            <div className="flex-center  mt-32">
                <div
                 onClick={gotoInstall}
                 className="btn-dan w-60 h-12">Install OpenDAN Desktop</div>
            </div>
            <div className="flex-center text-sm mt-4 text-gray-400">
            OpenDAN Desktop version {version}, build {build}
            </div>
            <div className="flex-center text-sm mt-1 text-gray-400">
                Copyright opendan.ai
            </div>
        </>
    )
}


export default Index