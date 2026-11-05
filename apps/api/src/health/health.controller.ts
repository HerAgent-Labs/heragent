import { Controller, Get } from '@nestjs/common';
import {
  ApiOperation,
  ApiResponse,
  ApiTags,
} from '@nestjs/swagger';

import { HealthService } from './health.service';

@ApiTags('Health')
@Controller('health')
export class HealthController {
  constructor(private readonly healthService: HealthService) {}

  @Get()
  @ApiOperation({ summary: 'Check HerAgent API health' })
  @ApiResponse({ status: 200, description: 'API is operational' })
  getHealth() {
    return this.healthService.getHealth();
  }
}