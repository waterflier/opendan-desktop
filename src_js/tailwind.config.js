/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/layout/**/*.{js,jsx,ts,tsx}', './src/pages/**/*.{js,jsx,ts,tsx}', './src/component/**/*.{js,jsx,ts,tsx}'],
    theme: {
        container: {
            center: true,
            padding: {
                DEFAULT: '0.5rem',
                md: '2.5rem'
            }
        },

        extend: {
            screens: {
                '3xl': '1920px'
                // short: { 'raw': '(max-width: 480px)' },
                // 'short': {'min': '480px'},
            },
            fontFamily: {
                Futura: ['Futura']
            },
            colors: {
                dan: {
                    red2: '#FF3353',
                    blue2: '#29ABFF',
                    green: '#52C41A',
                    'blue-100': 'rgb(205 235 254)',
                    blue1: '#91D5FF'
                }
            }
        }
    },
    plugins: []
};
