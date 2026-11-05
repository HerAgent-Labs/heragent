import { ValidationPipe } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { NestFactory } from '@nestjs/core';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';
import helmet from 'helmet';

import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  const configService = app.get(ConfigService);

  // API versioning
  app.setGlobalPrefix('api/v1');

  // Security headers
  app.use(helmet());

  // Cross-origin configuration
  const corsOrigins =
    configService.get<string>('cors.origin')?.split(',') ?? [];

  app.enableCors({
    origin: corsOrigins.map((origin) => origin.trim()),
    credentials: true,
  });

  // Global request validation
  app.useGlobalPipes(
    new ValidationPipe({
      whitelist: true,
      forbidNonWhitelisted: true,
      transform: true,
    }),
  );

  // Swagger documentation
  const swaggerConfig = new DocumentBuilder()
    .setTitle('HerAgent API')
    .setDescription(
      'API documentation for the HerAgent AI-powered business assistant.',
    )
    .setVersion(configService.get<string>('app.version') || '0.1.0')
    .addBearerAuth()
    .build();

  const document = SwaggerModule.createDocument(app, swaggerConfig);

  SwaggerModule.setup('api/docs', app, document);

  // Start server
  const port = configService.get<number>('app.port') || 4000;

  await app.listen(port);

  console.log(`HerAgent API running on http://localhost:${port}`);
  console.log(`Swagger documentation: http://localhost:${port}/api/docs`);
}

bootstrap();