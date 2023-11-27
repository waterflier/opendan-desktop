import './global.css';
import './common.css';
import { createRoot } from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';
import router from './routes'


const dom = document.getElementById('main');
if (dom) {
    createRoot(dom).render(
        <RouterProvider router={router} />
    );
}