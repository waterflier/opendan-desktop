import { useState } from "react"

const Installed = () => {
    const [version, setVersion] = useState('0.5.3')
    const [current, setCurrent] = useState('0.5.2')
    const [build, setBuild] = useState('2023-11-21')

    return (
        <>
            <h1 className="text-2xl">It was detected that you hava installed OpenDan desktop, you want:</h1>

            <div className="mt-20 flex items-center">
                <input className="w-4 h-4" type="radio" name='install'/>

                <label className="ml-2">
                    Upgrade to lastest version ({version})
                </label>
            </div>
            <div className="ml-6 mt-2 text-sm text-gray-400">installed: {current}, build {build}</div>

            <div className="mt-8 flex items-center">
                <input className="w-4 h-4" type="radio" name='install'/>

                <label className="ml-2">
                    Uninstall OpenDAN Desktop
                </label>
            </div>

            <div className="mt-10 flex items-center">
                <input className="w-4 h-4" type="radio" name='install'/>

                <label className="ml-2">
                    Open OpenDAN Dashboard
                </label>
            </div>

            <div className="flex-center mt-20 gap-10">
            <div className="btn-dan w-36 h-9">End</div>
            <div className="btn-dan w-36 h-9">Next</div>
            </div>
        </>
    )
}


export default Installed