import { createLogger, format, transports } from 'winston';

const { combine, timestamp, printf, colorize } = format;

// Custom format for log messages
const customFormat = printf(({ level, message, timestamp, ...metadata }) => {
    let msg = `${timestamp} [${level}]: ${message}`;

    // Add metadata if present
    if (Object.keys(metadata).length > 0) {
        msg += ` ${JSON.stringify(metadata)}`;
    }

    return msg;
});

// Create the logger instance
const logger = createLogger({
    format: combine(
        timestamp(),
        customFormat
    ),
    transports: [
        // Console transport with colors for development
        new transports.Console({
            format: combine(
                colorize(),
                customFormat
            )
        }),
        // File transport for errors
        new transports.File({
            filename: 'logs/error.log',
            level: 'error'
        }),
        // File transport for all logs
        new transports.File({
            filename: 'logs/combined.log'
        })
    ]
});

export default logger; 