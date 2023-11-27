import { Loader } from 'react-feather';


const Installing = () => {

    return (
        <>
            <div className='flex flex-col items-center justify-between h-[500px]'>

                <h1 className="text-center text-2xl">  OpenDAN is under installation</h1>

                <div className='flex flex-col items-center'>
                    <Loader className='text-dan-blue1 animate-spin' size={60} />
                    <div>Downloading Docker Desktop</div>
                </div>
                <div className="flex-center">
                    <div className="btn-dan w-36 h-9">Cancel</div>
                </div>
            </div>

        </>
    )
}


export default Installing