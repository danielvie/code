import logging
from TextColor import TextColor

# Create a custom logger
logger = logging.getLogger('mylogger')
logger.setLevel(logging.DEBUG)

# Create a console handler
console_handler = logging.StreamHandler()

# Create a custom formatter with ANSI escape codes
class CustomFormatter(logging.Formatter):
    def format(self, record):
        message = super().format(record)
        if 'DEBUG' in message:
            message = message.replace('DEBUG', f'{TextColor.CYAN.value}[D]{TextColor.CLEAR.value}')
        if 'INFO' in message:
            message = message.replace('INFO', f'{TextColor.GRAY.value}[I]{TextColor.CLEAR.value}')
        if 'WARNING' in message:
            message = message.replace('WARNING', f'{TextColor.YELLOW.value}[W]')
            message += TextColor.CLEAR.value
        if 'ERROR' in message:
            message = message.replace('ERROR', f'{TextColor.RED.value}[E]')
            message += TextColor.CLEAR.value
        if 'CRITICAL' in message:
            message = message.replace('CRITICAL', f'{TextColor.BG_RED.value}[CRITICAL]{TextColor.CLEAR.value}{TextColor.RED.value}')
            message += TextColor.CLEAR.value
        return message

# Create and set a formatter with custom coloring
formatter = CustomFormatter(
    '%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)
console_handler.setFormatter(formatter)

# Add the handler to the logger
logger.addHandler(console_handler)

# # Log messages with different severities
logger.debug('This is a debug message.')
logger.info('This is an info message.')
logger.warning('This is a warning message.')
logger.error('This is an error message.')
logger.critical('This is a critical message.')
