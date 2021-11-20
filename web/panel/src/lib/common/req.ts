import axios from 'axios'

import {API_HOST} from './config'

export interface iResp<T> {
	code: number
	data: T
	msg: string
}

const req = axios.create({
	baseURL: `//${API_HOST}`, // api的base_url
	timeout: 5000 // request timeout
})

// http 请求拦截器
req.interceptors.request.use(
	config => {
		// config.headers.common.Token = localStorage.getItem('XK-Token')
		return config
	},
	error => {
		return Promise.reject(error)
	}
)

// http 响应拦截器
req.interceptors.response.use(
	response => {
		const code = response?.data?.code
		// 401001 token过期
		if (code === 401001) {
			// token 过期提示并自动退出登录
			localStorage.removeItem('Token')
			// window.vueApp.$toast.removeAllGroups()
			// window.vueApp.$toast.add({
			// 	summary: '登录信息已过期，请重新登录',
			// 	severity: 'error',
			// 	life: 3000,
			// 	group: 'XK_GLOBAL_TOAST',
			// 	closeCallback: () => {
			// 		location.href = '/'
			// 	}
			// })
		}
		return response?.data
	},
	error => {
		return Promise.reject(error)
	}
)

export default req
