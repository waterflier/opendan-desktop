import {  Check } from 'react-feather'

const InstallSuccess = () => {
    return (
        <div className='flex flex-col items-center justify-between h-[500px]'>

        <h1 className="text-center text-2xl">Successful installation</h1>

        <div className='flex flex-col items-center'>
            <Check className='text-dan-green' size={100} />
            <div>OpenDAN is running on your computer</div>
        </div>
        <div className="flex-center">
            <div className="btn-dan w-48 h-10">Open Dashboard()</div>
        </div>
    </div>
    )
}


export default InstallSuccess