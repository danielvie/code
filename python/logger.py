import logging
import re
from colorama import init, Fore, Back, Style

init()
class CustomFormatter(logging.Formatter):
    # Define format strings for each log level
    FORMATS = {
        logging.DEBUG: f'%(asctime)s - {Fore.CYAN}[D] - %(filename)s:%(funcName)s - %(message)s{Style.RESET_ALL}',
        logging.INFO: f'%(asctime)s - {Fore.BLACK}[I] - %(filename)s:%(funcName)s - %(message)s{Style.RESET_ALL}',
        logging.WARNING: f'%(asctime)s - {Fore.YELLOW}[W] - %(filename)s:%(funcName)s - %(message)s{Style.RESET_ALL}',
        logging.ERROR: f'%(asctime)s - {Back.RED}[E]{Style.RESET_ALL}{Fore.RED} - %(filename)s:%(funcName)s - %(message)s{Style.RESET_ALL}',
    }

    def format(self, record):
        # Use the format string corresponding to the log level
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)

class FileFormatter(logging.Formatter):
    # Define format strings for each log level
    FORMATS = {
        logging.DEBUG: f'%(asctime)s - [D] - %(filename)s:%(funcName)s - %(message)s',
        logging.INFO: f'%(asctime)s - [I] - %(filename)s:%(funcName)s - %(message)s',
        logging.WARNING: f'%(asctime)s - [W] - %(filename)s:%(funcName)s - %(message)s',
        logging.ERROR: f'%(asctime)s - [E] - %(filename)s:%(funcName)s - %(message)s',
    }

    def format(self, record):
        # Use the format string corresponding to the log level
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)

# Create a logger
logger_name = 'logger    ^&*(^)132468'
logger_name = re.sub(r'\W', '_', logger_name)
logger_name = re.sub(r'_+', '_', logger_name)
logger = logging.getLogger(logger_name)
logger.setLevel(logging.DEBUG)

# Create a console handler
console_handler = logging.StreamHandler()
console_handler.setFormatter(CustomFormatter())

# Create a file handler
file_handler = logging.FileHandler(f'{logger_name}.log')
file_handler.setFormatter(FileFormatter())

# Add the handler to the logger
logger.addHandler(console_handler)
logger.addHandler(file_handler)