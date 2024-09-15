from logger import LoggerBuilder

logger = LoggerBuilder('test')
logger2 = LoggerBuilder('test2')

def example_function():
    logger.debug("This is a debug message for log")
    logger.info("This is an info message for log")
    logger.warning("This is a warning message for log")
    logger.error("This is an error message for log")
    logger2.error("This is an error message for log2")

def fun2():
    logger2.error("This is an error message for log2")

example_function()
fun2()
