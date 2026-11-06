import { Injectable } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class HealthService {
  private readonly startedAt = new Date();

  constructor(private readonly configService: ConfigService) {}

  getHealth() {
    return {
      status: 'okay',
      service: this.configService.get<string>('app.name'),
      version: this.configService.get<string>('app.version'),
      environment: this.configService.get<string>('app.environment'),
      uptime: Math.floor(process.uptime()),
      startedAt: this.startedAt.toISOString(),
      timestamp: new Date().toISOString(),
    };
  }
}