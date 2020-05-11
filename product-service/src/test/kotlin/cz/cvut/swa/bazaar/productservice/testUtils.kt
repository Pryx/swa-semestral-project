package cz.cvut.swa.bazaar.productservice

import cz.cvut.swa.bazaar.productservice.data.*
import java.math.BigDecimal
import java.time.LocalDateTime
import java.util.*

fun randomUuid() = UUID.randomUUID().toString()

fun fakeProduct(): Product = Product(
        randomUuid(),
        "testProduct",
        "Cool product",
        BigDecimal.ONE,
        randomUuid())

fun fakeProductDTO(): ProductDTO = ProductDTO(
        "testProduct",
        "Cool product",
        BigDecimal.ONE,
        randomUuid())

fun fakeReviewDTO(): ReviewDTO = ReviewDTO(
        randomUuid(),
        randomUuid(),
        "The product works as expected",
        LocalDateTime.now())

fun fakeProductWithReviewsDTO(): ProductWithReviewsDTO = ProductWithReviewsDTO(
        randomUuid(),
        "testProduct",
        "Cool product",
        BigDecimal.ONE,
        randomUuid(),
        ProductStatus.AVAILABLE,
        LocalDateTime.now(),
        emptyList()
)

fun fakeProducStatusUpdateDTO(newStatus: ProductStatus): ProductStatusUpdateDTO = ProductStatusUpdateDTO(
        randomUuid(),
        newStatus
)