package cz.cvut.swa.bazaar.productservice.data

import org.springframework.http.HttpStatus
import org.springframework.web.bind.annotation.ResponseStatus

@ResponseStatus(value = HttpStatus.NOT_FOUND)
class EntityNotFoundException(
        override val message: String? = "",
        override val cause: Throwable? = null) : RuntimeException(message, cause)

@ResponseStatus(value = HttpStatus.SERVICE_UNAVAILABLE)
class NetworkException(
        override val message: String? = "",
        override val cause: Throwable? = null) : RuntimeException(message, cause)