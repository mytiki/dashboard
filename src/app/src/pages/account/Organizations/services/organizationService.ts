import { RequestService } from '@/services'

export interface Organization {
  orgId: string
  name: string
  modified: string
  created: string
}

export class OrganizationService {
  static requestService = new RequestService()

  static async get(): Promise<Organization[]> {
    return await OrganizationService.requestService.get<Organization[]>('/account/org')
  }

  static async create(name: string): Promise<Organization> {
    return await OrganizationService.requestService.post<Organization>('/account/org', { name })
  }

  static async delete(orgId: string): Promise<void> {
    return await OrganizationService.requestService.delete<void>(`/account/org/${orgId}`)
  }
}
