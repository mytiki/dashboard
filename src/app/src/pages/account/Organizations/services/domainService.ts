import { RequestService } from '@/services'

export interface Domain {
  domainId: string
  hostname: string
  secret?: string
  created: string
  modified: string
  orgId?: string
}

export class DomainService {
  static requestService = new RequestService()

  static async get(): Promise<Domain[]> {
    return await DomainService.requestService.get<Domain[]>('/account/domain')
  }

  static async create(hostname: string, orgId: string): Promise<Domain> {
    return await DomainService.requestService.post<Domain>('/account/domain', { hostname, orgId })
  }

  static async getSecret(domainId: string): Promise<Domain> {
    return await DomainService.requestService.get<Domain>(`/account/domain/${domainId}?secret=true`)
  }

  static async refreshSecret(domainId: string): Promise<Domain> {
    return await DomainService.requestService.put<Domain>(`/account/domain/${domainId}/secret`)
  }

  static async delete(domainId: string): Promise<void> {
    return await DomainService.requestService.delete<void>(`/account/domain/${domainId}`)
  }
}
