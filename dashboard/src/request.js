import axios from 'axios'

import {
    get as getToken
} from './token'

export default axios.create({
    baseURL: '/api/',
    timeout: 5000,
    withCredentials: true,
    headers: {
        'Accept': 'application/json',
        'Authorization': `Bearer ${getToken()}`,
        'Content-Type': 'application/json; charset=utf-8',
    }
});