import {createBrowserRouter} from 'react-router-dom'
import Layout from '@layout/Layout'
import Index from '@pages/index/Index'
import Installed from '@pages/installed/Installed'

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
        ]
    }
]);

export default router;
