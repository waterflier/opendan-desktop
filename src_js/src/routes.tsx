import {createBrowserRouter} from 'react-router-dom'
import Layout from '@layout/Layout'
import Index from '@pages/index/Index'
import Installed from '@pages/installed/Installed'
import Installing from '@pages/installing/Installing'

const router = createBrowserRouter([
    {
        path: '/',
        element: <Layout />,
        children: [
            {
                path: '/',
                element: <Index />
            },
            {
                path: '/installed',
                element: <Installed />
            },
            {
                path: '/installing',
                element: <Installing />
            },
        ]
    }
]);

export default router;
