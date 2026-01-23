"""
@file celery_worker.py
@description Celery worker entrypoint to run retraining tasks
"""

from retrain import app

if __name__ == '__main__':
    app.worker_main()
