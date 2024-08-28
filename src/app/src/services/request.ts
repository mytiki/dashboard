// import { auth } from './'

// enum HttpClientBaseMethod {
//   POST = 'POST',
//   GET = 'GET',
//   PUT = 'PUT'
// }

// export default class RequestService {
//   protected static _apiUrl = import.meta.env.VITE_API_URL

//   // Private method to handle responses
//   static async handleResponse<T>(response: Response): Promise<T> {
//     if (!response.ok) {
//       const errorDetails = await response.json().catch(() => ({}))
//       throw new Error(
//         `Error: ${response.status} ${response.statusText} - ${JSON.stringify(errorDetails)}`
//       )
//     }
//     return response.json()
//   }

//   // Private method to get headers with token
//   private static async getHeaders(): Promise<Headers> {
//     const headers = new Headers()
//     try {
//       const token = await auth.token()
//       headers.append('Authorization', 'Bearer ' + token)
//       headers.append('Content-Type', 'application/json')
//     } catch (error) {
//       throw new Error('Unable to retrieve authentication token')
//     }
//     return headers
//   }

//   // Private method to make HTTP requests
//   static async request<T>(
//     endpoint: string,
//     method: HttpClientBaseMethod,
//     body?: object
//   ): Promise<T> {
//     const url = `${RequestService._apiUrl}${endpoint}`
//     const headers = await RequestService.getHeaders()

//     const options: RequestInit = {
//       method: method,
//       headers: headers,
//       body: body ? JSON.stringify(body) : undefined
//     }

//     try {
//       const response = await fetch(url, options)
//       return await RequestService.handleResponse<T>(response)
//     } catch (error) {
//       console.error(`Error ${method.toLowerCase()}ing data:`, error)
//       throw new Error(`Failed to ${method.toLowerCase()} data.`)
//     }
//   }

//   // Public method to make GET requests
//   protected static get<T>(endpoint: string): Promise<T> {
//     return RequestService.request<T>(endpoint, HttpClientBaseMethod.GET)
//   }

//   // Public method to make POST requests
//   protected static post<T>(endpoint: string, body: object): Promise<T> {
//     return RequestService.request<T>(endpoint, HttpClientBaseMethod.POST, body)
//   }
// }

import { auth } from './'

export default class RequestService {
  protected _apiUrl = import.meta.env.VITE_API_URL

  // Method to handle responses
  protected async handleResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
      const errorDetails = await response.json().catch(() => ({}))
      throw new Error(
        `Error: ${response.status} ${response.statusText} - ${JSON.stringify(errorDetails)}`
      )
    }
    return response.json()
  }

  // Method to get headers with token
  private async getHeaders(): Promise<Headers> {
    const headers = new Headers()
    try {
      const token = await auth.token()
      headers.append('Authorization', 'Bearer ' + token)
      headers.append('Content-Type', 'application/json')
    } catch (error) {
      throw new Error('Unable to retrieve authentication token')
    }
    return headers
  }

  // Method to make HTTP requests
  protected async request<T>(endpoint: string, method: 'GET' | 'POST', body?: object): Promise<T> {
    const url = `${this._apiUrl}${endpoint}`
    const headers = await this.getHeaders()

    const options: RequestInit = {
      method: method,
      headers: headers,
      body: body ? JSON.stringify(body) : undefined
    }

    try {
      const response = await fetch(url, options)
      return await this.handleResponse<T>(response)
    } catch (error) {
      console.error(`Error ${method.toLowerCase()}ing data:`, error)
      throw new Error(`Failed to ${method.toLowerCase()} data.`)
    }
  }

  // Public method to make GET requests
  public async get<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, 'GET')
  }

  // Public method to make POST requests
  public async post<T>(endpoint: string, body: object): Promise<T> {
    return this.request<T>(endpoint, 'POST', body)
  }
}
